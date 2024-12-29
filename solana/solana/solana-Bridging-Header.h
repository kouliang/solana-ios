//
//  Bridging-Header.h
//  solana
//
//  Created by kou on 2024/12/28.
//

#ifndef Bridging_Header_h
#define Bridging_Header_h

#include <stdint.h>

const char* test_rpc(const char* content);
const char* test_key_pair(const char* content);

const char* save_config(const char* rpc, const char* keypair);

const char* balance(const char* content);
const char* transfer_to(const char* address, const char* amount);

#endif /* Bridging_Header_h */
