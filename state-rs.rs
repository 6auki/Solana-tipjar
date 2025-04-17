use anchor_lang::prelude::*;

/// The main struct that stores all tip jar data on-chain
/// This is created as a PDA owned by the program
#[account]
pub struct TipJar {
    /// Whether tips can currently be sent to this jar
    pub is_active: bool,
    /// Whether this jar is private (only owner can send tips)
    pub is_private: bool,
    /// The wallet that owns this tip jar and can withdraw funds
    pub owner: Pubkey,
    /// Description of what this tip jar is for
    pub description: String,
    /// Category tag for the tip jar (e.g., "content creator", "charity")
    pub category: String,
    /// Fundraising goal amount in lamports (1 SOL = 1,000,000,000 lamports)
    pub goal: u64,
    /// Total amount of SOL received in lamports
    pub total_received: u64,
    /// History of recent tips, implemented as a circular buffer
    pub tips_history: Vec<Tip>,
    /// Current position in the circular buffer
    pub last_tip_index: u16,
    /// Total count of all tips ever received (not limited by buffer size)
    pub total_tips_count: u32,
    /// PDA bump used to derive this account's address
    pub bump: u8,
}

/// Implementation for TipJar with space calculation and constants
impl TipJar {
    // Base account discriminator - Anchor uses this to identify account types
    const DISCRIMINATOR_SIZE: usize = 8;
    
    // Static fields total size
    const STATIC_SIZE: usize = 
        1 + // is_active
        1 + // is_private
        32 + // owner (Pubkey)
        8 + // goal
        8 + // total_received
        1 + // bump
        2 + // last_tip_index
        4; // total_tips_count
    
    // Dynamic fields calculation
    const MAX_DESCRIPTION_LEN: usize = 200;
    const MAX_CATEGORY_LEN: usize = 100;
    /// Maximum number of tips to store in history
    pub const MAX_TIP_HISTORY_LEN: usize = 100; // Reduced for efficient space usage
    
    /// Calculates the total space needed for this account
    pub fn space() -> usize {
        Self::DISCRIMINATOR_SIZE +        // Account discriminator
        Self::STATIC_SIZE +               // Fixed-size fields
        4 + Self::MAX_DESCRIPTION_LEN +   // String prefix (4) + max chars
        4 + Self::MAX_CATEGORY_LEN +      // String prefix (4) + max chars
        4 + (Self::MAX_TIP_HISTORY_LEN * Tip::SIZE) // Vec prefix (4) + entries
    }
    
    /// Total length constant used in account initialization
    pub const LEN: usize = Self::space();
}

/// Represents a single tip with sender, amount, and message
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Tip {
    /// Public key of the tip sender
    pub sender: Pubkey,
    /// Amount of SOL sent in lamports
    pub amount: u64,
    /// Whether this tip is publicly visible or anonymous
    pub visibility: Visibility,
    /// Optional message included with the tip
    pub memo: String,
    /// Unix timestamp when the tip was sent
    pub timestamp: u64,
}

/// Implementation for Tip with space calculation
impl Tip {
    /// Size of a single tip in bytes
    pub const SIZE: usize = 32 + // sender (Pubkey)
                            8 +  // amount (u64)
                            1 +  // visibility (enum)
                            (4 + 100) + // memo (String with max 100 chars)
                            8;  // timestamp (u64)
}

/// Enum for tip visibility settings
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Visibility {
    /// Tip is publicly visible with sender info
    Public,
    /// Tip is anonymous (sender info hidden)
    Anonymous,
}