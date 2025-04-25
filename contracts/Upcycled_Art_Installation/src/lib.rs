#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Structure to store information about an art piece
#[contracttype]
#[derive(Clone)]
pub struct ArtPiece {
    pub id: u64,
    pub title: String,
    pub artist: String,
    pub materials: String,
    pub creation_timestamp: u64,
    pub verified: bool,
}

// For creating unique art piece IDs
const ART_COUNT: Symbol = symbol_short!("ART_CNT");

// Mapping art piece to its ID
#[contracttype]
pub enum ArtRegistry {
    ArtPiece(u64)
}

// Summary data structure for platform statistics
#[contracttype]
#[derive(Clone)]
pub struct PlatformStats {
    pub total_pieces: u64,
    pub verified_pieces: u64,
}

// Key for storing platform stats
const PLATFORM_STATS: Symbol = symbol_short!("STATS");

#[contract]
pub struct UpcycledArtContract;

#[contractimpl]
impl UpcycledArtContract {
    // Register a new upcycled art piece
    pub fn register_art(env: Env, title: String, artist: String, materials: String) -> u64 {
        // Get current art count and increment
        let mut art_count: u64 = env.storage().instance().get(&ART_COUNT).unwrap_or(0);
        art_count += 1;
        
        // Create new art piece
        let timestamp = env.ledger().timestamp();
        let new_art = ArtPiece {
            id: art_count,
            title: title,
            artist: artist,
            materials: materials,
            creation_timestamp: timestamp,
            verified: false,
        };
        
        // Update platform stats
        let mut stats = Self::get_platform_stats(env.clone());
        stats.total_pieces += 1;
        
        // Store art piece and updated stats
        env.storage().instance().set(&ArtRegistry::ArtPiece(art_count), &new_art);
        env.storage().instance().set(&ART_COUNT, &art_count);
        env.storage().instance().set(&PLATFORM_STATS, &stats);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Art piece registered with ID: {}", art_count);
        return art_count;
    }
    
    // Verify an art piece (to be called by authorized curator/admin)
    pub fn verify_art(env: Env, art_id: u64) {
        let mut art_piece = Self::get_art_piece(env.clone(), art_id);
        
        if art_piece.id == 0 {
            log!(&env, "Art piece with ID {} not found", art_id);
            panic!("Art piece not found");
        }
        
        // Only verify if not already verified
        if !art_piece.verified {
            art_piece.verified = true;
            
            // Update platform stats
            let mut stats = Self::get_platform_stats(env.clone());
            stats.verified_pieces += 1;
            
            // Store updated art piece and stats
            env.storage().instance().set(&ArtRegistry::ArtPiece(art_id), &art_piece);
            env.storage().instance().set(&PLATFORM_STATS, &stats);
            
            env.storage().instance().extend_ttl(5000, 5000);
            
            log!(&env, "Art piece ID: {} is now verified", art_id);
        } else {
            log!(&env, "Art piece ID: {} is already verified", art_id);
        }
    }
    
    // Get information about an art piece by ID
    pub fn get_art_piece(env: Env, art_id: u64) -> ArtPiece {
        let key = ArtRegistry::ArtPiece(art_id);
        
        env.storage().instance().get(&key).unwrap_or(ArtPiece {
            id: 0,
            title: String::from_str(&env, "Not Found"),
            artist: String::from_str(&env, "Unknown"),
            materials: String::from_str(&env, "Unknown"),
            creation_timestamp: 0,
            verified: false,
        })
    }
    
    // Get platform statistics
    pub fn get_platform_stats(env: Env) -> PlatformStats {
        env.storage().instance().get(&PLATFORM_STATS).unwrap_or(PlatformStats {
            total_pieces: 0,
            verified_pieces: 0,
        })
    }
}
