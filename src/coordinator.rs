//! FROSTR Coordinator
//!
//! The coordinator runs an event loop that broadcasts and receives messages to orchestrate two workflows:
//! - key generation
//! - signing
//!
//! The Key Generation workflow has a few steps:
//! 1. Generate a keypair for this quorum and a list of invite codes, return those to the caller
//! 2. The caller distributes invite codes and the coordinator pubkey through whatever method
//! 3. Each participant registers their pubkey by sending a message to the coordinator key with the invite code
//! 4. The coordinator marks the invite code as "used" (can only be used once) and then assigns the participant pubkey to a participant index
//! 5. Once all of the invite codes have been used up, the coordinator sends each participant the list of participant indexes mapped to pubkeys
//! 6. Each participant sends its polynomials to the coordinator
//! 7. The coordinator sends each participant polynomial to each other participant
//! 8. Each participant then can trade per-index secret shares and possession proofs with each other
//! 9. Each participant sends its computed aggregate pubkey to the coordinator
//! 10. The coordinator makes sure they all match, broadcasts that key generation is complete
//!
//! The Signing workflow has a few steps:
//! 1. A participant sends a message and a timestamp to the coordinator's qurorum-specific pubkey
//! 2. The coordinator assembles the event, sends the hash, message, and timestamp to each participant
//! 3. Each participant who wants to participate sends their nonce commitments to the coordinator
//! 4. The coordinator sends the nonce commitments and indecies to each participating participant
//! 5. Each participant sends a signature share to the coordinator
//! 6. The coordinator aggregates all the signature shares and sends the final signature and a status code to the participants
//! 7. The coordinator broadcasts the message.

pub(crate) struct Coordinator {

}

