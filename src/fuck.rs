//! Either brain or mushroom. Up to you.

                                            use std::fs;use std::io::Read;
                                        const FUCK_YOU:&str="FUCK YOU";fn//
                                     main(){let mut fucking_bytes=[0u8;30000];//
                                    let mut fucking_head=0usize;let code=std::env::
                                 args().collect::<Vec<_>>();let code=fs::read_to_string
                            (code.get(1).expect(FUCK_YOU)).expect(FUCK_YOU);let mut i=0;while
                         i<code.len() {let c=code.chars().nth(i).unwrap();match c{'+'=>/*fuuuck*/
                     fucking_bytes[fucking_head]=fucking_bytes[fucking_head].overflowing_add(1).0,'-'
                    =>{let aa=fucking_bytes[fucking_head].overflowing_sub(1).0;fucking_bytes[fucking_head
                    ]=aa;}'>'=>{fucking_head+=1;fucking_head%=30000;},'<'=>{fucking_head-=1;if/*fuckfuckf*/
                    fucking_head>30000{fucking_head=29999;}},'.'=>print!("{}", fucking_bytes[fucking_head]
                      as char),','=>fucking_bytes[fucking_head]=std::io::stdin().bytes().next().expect(//
                        FUCK_YOU).expect(FUCK_YOU),'[' if fucking_bytes[fucking_head]==0=>{let mut//
                            occured=0;for (_i,c) in code.chars().skip(i+1).enumerate(){match c{
                              '['=>occured+=1,']' if occured!=0=>occured-=1,']'=>{i+=_i+1;
                                 break}_=>continue,}}continue}']' if fucking_bytes
                                   [fucking_head]!=0=>{let len=code.len();
                                                        let mut occured=0;for (_i, c)
                                                        in code.chars().rev().skip(len-i)
                                                          .enumerate() {match c{']'=>occured
                                                          +=1,'[' if occured!=0=>occured-=1,
                                                           '['=>{i-=_i + 1;break}_=>continue,
                                                                 }}continue}_=>(),}i+=1;}}