//! # CBOR Tags Registry
//!
//! https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2020-006-urtypes.md
//!
//! As of August 13 2022, the [IANA registry of CBOR tags](https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml)
//! has the following low-numbered values available:
//!
//! One byte encoding: 6-15, 19-20
//! Two byte encoding: 48-51, 53, 55-60, 62, 88-95, 99, 102, 105-109, 113-119, 128-255
//!
//! Tags in the range 0-23 require "standards action" for the IANA to recognize.
//! Tags in the range 24-32767 require a specification to reserve.
//! Tags in the range 24-255 only require two bytes to encode.
//! Higher numbered tags are first-come, first-served.

use dcbor::prelude::*;

pub use dcbor::TAG_DATE;

const_cbor_tag!(32, URI, "url");
const_cbor_tag!(37, UUID, "uuid");

// A previous version of the Envelope spec used tag #6.24 ("Encoded CBOR Item") as
// the header for the Envelope `leaf` case. Unfortunately, this was not a correct
// use of the tag, as the contents of #6.24 (RFC8949 §3.4.5.1) MUST always be a
// byte string, while we were simply using it as a wrapper/header for any dCBOR
// data item.
//
// https://www.rfc-editor.org/rfc/rfc8949.html#name-encoded-cbor-data-item
//
// The new leaf tag is #6.201, but we will still recognize #6.24 for backwards
// compatibility.

// The only two tags that Blockchain Commons has registered in the "Specification Required"
// range are the two tags for "Gordian Envelope" (#6.200) and "dCBOR/Envelope Leaf" (#6.201).

//
// Core Envelope tags.
//

const_cbor_tag!(24, ENCODED_CBOR, "encoded-cbor");
const_cbor_tag!(200, ENVELOPE, "envelope");
const_cbor_tag!(201, LEAF, "leaf");

//
// Envelope extension tags
//

const_cbor_tag!(40000, KNOWN_VALUE, "known-value");
const_cbor_tag!(40001, DIGEST, "digest");
const_cbor_tag!(40002, ENCRYPTED, "encrypted");
const_cbor_tag!(40003, COMPRESSED, "compressed");

//
// Tags for subtypes specific to Distributed Function Calls.
//

const_cbor_tag!(40004, REQUEST, "request");
const_cbor_tag!(40005, RESPONSE, "response");
const_cbor_tag!(40006, FUNCTION, "function");
const_cbor_tag!(40007, PARAMETER, "parameter");
const_cbor_tag!(40008, PLACEHOLDER, "placeholder");
const_cbor_tag!(40009, REPLACEMENT, "replacement");

const_cbor_tag!(40010, X25519_PRIVATE_KEY, "agreement-private-key");
const_cbor_tag!(40011, X25519_PUBLIC_KEY, "agreement-public-key");
const_cbor_tag!(40012, ARID, "arid");
const_cbor_tag!(40013, PRIVATE_KEYS, "crypto-prvkeys");
const_cbor_tag!(40014, NONCE, "nonce");
const_cbor_tag!(40015, PASSWORD, "password");
const_cbor_tag!(40016, PRIVATE_KEY_BASE, "crypto-prvkey-base");

const_cbor_tag!(40017, PUBLIC_KEYS, "crypto-pubkeys");
const_cbor_tag!(40018, SALT, "salt");
const_cbor_tag!(40019, SEALED_MESSAGE, "crypto-sealed");
const_cbor_tag!(40020, SIGNATURE, "signature");
const_cbor_tag!(40021, SIGNING_PRIVATE_KEY, "signing-private-key");
const_cbor_tag!(40022, SIGNING_PUBLIC_KEY, "signing-public-key");
const_cbor_tag!(40023, SYMMETRIC_KEY, "crypto-key");
const_cbor_tag!(40024, XID, "xid");
const_cbor_tag!(40025, REFERENCE, "reference");
const_cbor_tag!(40026, EVENT, "event");

const_cbor_tag!(40027, ENCRYPTED_KEY, "encrypted-key");
const_cbor_tag!(40100, MLKEM_PRIVATE_KEY, "mlkem-private-key");
const_cbor_tag!(40101, MLKEM_PUBLIC_KEY, "mlkem-public-key");
const_cbor_tag!(40102, MLKEM_CIPHERTEXT, "mlkem-ciphertext");
const_cbor_tag!(40103, MLDSA_PRIVATE_KEY, "mldsa-private-key");
const_cbor_tag!(40104, MLDSA_PUBLIC_KEY, "mldsa-public-key");
const_cbor_tag!(40105, MLDSA_SIGNATURE, "mldsa-signature");
const_cbor_tag!(40300, SEED, "seed");
const_cbor_tag!(40303, HDKEY, "hdkey");
const_cbor_tag!(40304, DERIVATION_PATH, "keypath");

const_cbor_tag!(40305, USE_INFO, "coin-info");
const_cbor_tag!(40306, EC_KEY, "eckey");
const_cbor_tag!(40307, ADDRESS, "address");
const_cbor_tag!(40308, OUTPUT_DESCRIPTOR, "output-descriptor");
const_cbor_tag!(40309, SSKR_SHARE, "sskr");
const_cbor_tag!(40310, PSBT, "psbt");
const_cbor_tag!(40311, ACCOUNT_DESCRIPTOR, "account-descriptor");
const_cbor_tag!(40800, SSH_TEXT_PRIVATE_KEY, "ssh-private");
const_cbor_tag!(40801, SSH_TEXT_PUBLIC_KEY, "ssh-public");
const_cbor_tag!(40802, SSH_TEXT_SIGNATURE, "ssh-signature");
const_cbor_tag!(40803, SSH_TEXT_CERTIFICATE, "ssh-certificate");

const_cbor_tag!(1347571542, PROVENANCE_MARK, "provenance");

//
// DEPRECATED TAGS
//
// The following tags are deprecated and should not be used in new code.
// Unfortunately, they are likely to be in active use by external developers,
// but should never have been used as they are in the range of CBOR tags
// requiring "Specification" action by IANA, which requires IANA experts to
// review and approve the specification. These are harder to get approved than
// "First Come First Served" tags, and we don't want to have to do that for
// every new tag we create. Most of these tags have been replaced by "First Come
// First Served" tags in the range of 40000+.
//

const_cbor_tag!(300, SEED_V1, "crypto-seed");
const_cbor_tag!(306, EC_KEY_V1, "crypto-eckey");
const_cbor_tag!(309, SSKR_SHARE_V1, "crypto-sskr");

const_cbor_tag!(303, HDKEY_V1, "crypto-hdkey");
const_cbor_tag!(304, DERIVATION_PATH_V1, "crypto-keypath");
const_cbor_tag!(305, USE_INFO_V1, "crypto-coin-info");
const_cbor_tag!(307, OUTPUT_DESCRIPTOR_V1, "crypto-output");
const_cbor_tag!(310, PSBT_V1, "crypto-psbt");
const_cbor_tag!(311, ACCOUNT_V1, "crypto-account");

// Tags for subtypes specific to AccountBundle (crypto-output).
const_cbor_tag!(400, OUTPUT_SCRIPT_HASH, "output-script-hash");
const_cbor_tag!(401, OUTPUT_WITNESS_SCRIPT_HASH, "output-witness-script-hash");
const_cbor_tag!(402, OUTPUT_PUBLIC_KEY, "output-public-key");
const_cbor_tag!(403, OUTPUT_PUBLIC_KEY_HASH, "output-public-key-hash");
const_cbor_tag!(404, OUTPUT_WITNESS_PUBLIC_KEY_HASH, "output-witness-public-key-hash");
const_cbor_tag!(405, OUTPUT_COMBO, "output-combo");
const_cbor_tag!(406, OUTPUT_MULTISIG, "output-multisig");
const_cbor_tag!(407, OUTPUT_SORTED_MULTISIG, "output-sorted-multisig");
const_cbor_tag!(408, OUTPUT_RAW_SCRIPT, "output-raw-script");
const_cbor_tag!(409, OUTPUT_TAPROOT, "output-taproot");
const_cbor_tag!(410, OUTPUT_COSIGNER, "output-cosigner");

pub fn register_tags_in(tags_store: &mut TagsStore) {
    dcbor::register_tags_in(tags_store);

    let tags = vec![
        cbor_tag!(URI),
        cbor_tag!(UUID),
        cbor_tag!(ENCODED_CBOR),
        cbor_tag!(ENVELOPE),
        cbor_tag!(LEAF),
        cbor_tag!(KNOWN_VALUE),
        cbor_tag!(DIGEST),
        cbor_tag!(ENCRYPTED),
        cbor_tag!(COMPRESSED),
        cbor_tag!(REQUEST),
        cbor_tag!(RESPONSE),
        cbor_tag!(FUNCTION),
        cbor_tag!(PARAMETER),
        cbor_tag!(PLACEHOLDER),
        cbor_tag!(REPLACEMENT),
        cbor_tag!(EVENT),
        cbor_tag!(SEED_V1),
        cbor_tag!(EC_KEY_V1),
        cbor_tag!(SSKR_SHARE_V1),
        cbor_tag!(SEED),
        cbor_tag!(EC_KEY),
        cbor_tag!(SSKR_SHARE),
        cbor_tag!(X25519_PRIVATE_KEY),
        cbor_tag!(X25519_PUBLIC_KEY),
        cbor_tag!(ARID),
        cbor_tag!(PRIVATE_KEYS),
        cbor_tag!(NONCE),
        cbor_tag!(PASSWORD),
        cbor_tag!(PRIVATE_KEY_BASE),
        cbor_tag!(PUBLIC_KEYS),
        cbor_tag!(SALT),
        cbor_tag!(SEALED_MESSAGE),
        cbor_tag!(SIGNATURE),
        cbor_tag!(SIGNING_PRIVATE_KEY),
        cbor_tag!(SIGNING_PUBLIC_KEY),
        cbor_tag!(SYMMETRIC_KEY),
        cbor_tag!(XID),
        cbor_tag!(REFERENCE),
        cbor_tag!(ENCRYPTED_KEY),
        cbor_tag!(MLKEM_PRIVATE_KEY),
        cbor_tag!(MLKEM_PUBLIC_KEY),
        cbor_tag!(MLKEM_CIPHERTEXT),
        cbor_tag!(MLDSA_PRIVATE_KEY),
        cbor_tag!(MLDSA_PUBLIC_KEY),
        cbor_tag!(MLDSA_SIGNATURE),
        cbor_tag!(HDKEY_V1),
        cbor_tag!(DERIVATION_PATH_V1),
        cbor_tag!(USE_INFO_V1),
        cbor_tag!(OUTPUT_DESCRIPTOR_V1),
        cbor_tag!(PSBT_V1),
        cbor_tag!(ACCOUNT_V1),
        cbor_tag!(HDKEY),
        cbor_tag!(DERIVATION_PATH),
        cbor_tag!(USE_INFO),
        cbor_tag!(ADDRESS),
        cbor_tag!(OUTPUT_DESCRIPTOR),
        cbor_tag!(PSBT),
        cbor_tag!(ACCOUNT_DESCRIPTOR),
        cbor_tag!(SSH_TEXT_PRIVATE_KEY),
        cbor_tag!(SSH_TEXT_PUBLIC_KEY),
        cbor_tag!(SSH_TEXT_SIGNATURE),
        cbor_tag!(SSH_TEXT_CERTIFICATE),
        cbor_tag!(OUTPUT_SCRIPT_HASH),
        cbor_tag!(OUTPUT_WITNESS_SCRIPT_HASH),
        cbor_tag!(OUTPUT_PUBLIC_KEY),
        cbor_tag!(OUTPUT_PUBLIC_KEY_HASH),
        cbor_tag!(OUTPUT_WITNESS_PUBLIC_KEY_HASH),
        cbor_tag!(OUTPUT_COMBO),
        cbor_tag!(OUTPUT_MULTISIG),
        cbor_tag!(OUTPUT_SORTED_MULTISIG),
        cbor_tag!(OUTPUT_RAW_SCRIPT),
        cbor_tag!(OUTPUT_TAPROOT),
        cbor_tag!(OUTPUT_COSIGNER),
        cbor_tag!(PROVENANCE_MARK),
    ];
    tags_store.insert_all(tags);
}

pub fn register_tags() {
    with_tags_mut!(|tags_store: &mut TagsStore| {
        register_tags_in(tags_store);
    });
}
