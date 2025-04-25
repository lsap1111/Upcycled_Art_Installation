#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short, Address};

// Define a structure for Tree Certificate
#[contracttype]
#[derive(Clone)]
pub struct TreeCertificate {
    pub certificate_id: u64,
    pub tree_owner: Address,
    pub tree_species: String,
    pub location: String,
    pub planting_date: u64,
    pub verified: bool,
}

// Store certificate count
const CERTIFICATE_COUNT: Symbol = symbol_short!("CERT_CNT");
// Mapping certificate to its ID
#[contracttype] 
pub enum CertificateBook { 
    Certificate(u64)
}

// Statistics tracking
#[contracttype]
#[derive(Clone)]
pub struct CertificateStats {
    pub total_certificates: u64,
    pub verified_certificates: u64,
    pub total_tree_species: u64,
}

// Stats storage key
const STATS: Symbol = symbol_short!("STATS");

#[contract]
pub struct TreePlantingCertificate;

#[contractimpl]
impl TreePlantingCertificate {
    // Create a new tree planting certificate
    pub fn plant_tree(env: Env, owner: Address, species: String, location: String) -> u64 {
        // Get current certificate count
        let mut cert_count: u64 = env.storage().instance().get(&CERTIFICATE_COUNT).unwrap_or(0);
        cert_count += 1;
        
        // Get current timestamp
        let planting_time = env.ledger().timestamp();
        
        // Create certificate
        let certificate = TreeCertificate {
            certificate_id: cert_count,
            tree_owner: owner,
            tree_species: species,
            location: location,
            planting_date: planting_time,
            verified: false,
        };
        
        // Update statistics
        let mut stats = Self::get_stats(env.clone());
        stats.total_certificates += 1;
        
        // Store certificate and updated data
        env.storage().instance().set(&CertificateBook::Certificate(cert_count), &certificate);
        env.storage().instance().set(&CERTIFICATE_COUNT, &cert_count);
        env.storage().instance().set(&STATS, &stats);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Tree certificate created with ID: {}", cert_count);
        
        return cert_count;
    }
    
    // Verify a tree planting certificate (admin function)
    pub fn verify_certificate(env: Env, certificate_id: u64) {
        // Get certificate
        let key = CertificateBook::Certificate(certificate_id);
        let mut certificate: TreeCertificate = env.storage().instance().get(&key)
            .expect("Certificate not found");
        
        // Update verification status
        if !certificate.verified {
            certificate.verified = true;
            
            // Update stats
            let mut stats = Self::get_stats(env.clone());
            stats.verified_certificates += 1;
            
            // Store updated data
            env.storage().instance().set(&key, &certificate);
            env.storage().instance().set(&STATS, &stats);
            
            env.storage().instance().extend_ttl(5000, 5000);
            
            log!(&env, "Certificate ID: {} has been verified", certificate_id);
        } else {
            log!(&env, "Certificate already verified");
        }
    }
    
    // View a certificate by ID
    pub fn view_certificate(env: Env, certificate_id: u64) -> TreeCertificate {
        let key = CertificateBook::Certificate(certificate_id);
        
        env.storage().instance().get(&key).unwrap_or(TreeCertificate {
            certificate_id: 0,
            tree_owner: Address::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"),
            tree_species: String::from_str(&env, "Not Found"),
            location: String::from_str(&env, "Not Found"),
            planting_date: 0,
            verified: false,
        })
    }
    
    // Get overall statistics
    pub fn get_stats(env: Env) -> CertificateStats {
        env.storage().instance().get(&STATS).unwrap_or(CertificateStats {
            total_certificates: 0,
            verified_certificates: 0,
            total_tree_species: 0,
        })
    }
}