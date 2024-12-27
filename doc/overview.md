# Prolock

## Data structures

    PlFile
        - stored: Stored
            * readable: Readable
                * header: FileHeader
                    * format_version: u8
                    * update_counter: Sequence<usize>
                * content: Bundles
                    * Bundles: BTreeMap<String, Bundle>
                        * Bundle
                            * description: String,
                            * named_secrets: BTreeMap<String, Secret>,
                                * enum Secret{New(String), Ref(u64)}
            * cipher: String
        - o_transient: Option<Transient>
            * storage_password: SecUtf8
            * seq_for_secret_refs: Sequence<u64>
            * secrets: Secrets(HashMap<u64, String>)

```mermaid
    flowchart LR

    subgraph stored: Stored
        direction LR
        subgraph readable: Readable
            subgraph header: FileHeader
                format_version
                update_counter
            end
            subgraph content: NamedBundles
                subgraph Bundle
                    direction LR
                    description
                    subgraph named_secrets
                        Secret:Ref
                    end
                end
            end
        end
        subgraph cipher
            C(unreadable ciphertext)
        end
    end

    subgraph transient
        storage_password
        seq_for_secret_refs
        Secrets:Hashmap
    end

    Secret:Ref -- u64 --> Secrets:Hashmap
    Secrets:Hashmap -. encrypt .-> cipher
    cipher -. decrypt .-> Secrets:Hashmap


classDef class1 fill:#ffc,stroke:#333,stroke-width:1px;
classDef class2 fill:#ffb,stroke:#333,stroke-width:1px;
classDef class3 fill:#ffa,stroke:#333,stroke-width:1px;
classDef class4 fill:#ff9,stroke:#333,stroke-width:1px;
class readable class1
class content:NamedBundles,header class2
class Bundle class3
class named_secrets class4
```

## Main flow

```mermaid
flowchart LR

FILE@{ shape: doc, label: "prolock file" } -- read --> S(content, 
cipher)

subgraph Main Flow
    direction RL
    S -- enter password,
    decrypt --> ST(content, 
    cipher, 
    transient)
    ST -- edit content --> SmodT(content*, 
    cipher, 
    transient*)
    SmodT -- encrypt+save --> SupT(content*, 
    cipher*, 
    transient*)
end
SupT --> FILE
```