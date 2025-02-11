import type * as __compactRuntime from '@midnight-ntwrk/compact-runtime';

export type Witnesses<T> = {
}

export type ImpureCircuits<T> = {
  spend(context: __compactRuntime.CircuitContext<T>,
        sk: { is_left: boolean,
              left: { bytes: Uint8Array },
              right: { bytes: Uint8Array }
            },
        path: { leaf: Uint8Array,
                path: { sibling: { field: bigint }, goes_left: boolean }[]
              },
        coin: { nonce: Uint8Array, color: Uint8Array, value: bigint },
        rc: bigint): __compactRuntime.CircuitResults<T, void>;
  output(context: __compactRuntime.CircuitContext<T>,
         pk: { is_left: boolean,
               left: { bytes: Uint8Array },
               right: { bytes: Uint8Array }
             },
         coin: { nonce: Uint8Array, color: Uint8Array, value: bigint },
         rc: bigint): __compactRuntime.CircuitResults<T, void>;
}

export type PureCircuits = {
}

export type Circuits<T> = {
  spend(context: __compactRuntime.CircuitContext<T>,
        sk: { is_left: boolean,
              left: { bytes: Uint8Array },
              right: { bytes: Uint8Array }
            },
        path: { leaf: Uint8Array,
                path: { sibling: { field: bigint }, goes_left: boolean }[]
              },
        coin: { nonce: Uint8Array, color: Uint8Array, value: bigint },
        rc: bigint): __compactRuntime.CircuitResults<T, void>;
  output(context: __compactRuntime.CircuitContext<T>,
         pk: { is_left: boolean,
               left: { bytes: Uint8Array },
               right: { bytes: Uint8Array }
             },
         coin: { nonce: Uint8Array, color: Uint8Array, value: bigint },
         rc: bigint): __compactRuntime.CircuitResults<T, void>;
}

export type Ledger = {
  merkleTree: {
    isFull(): boolean;
    checkRoot(rt: { field: bigint }): boolean;
    root(): __compactRuntime.MerkleTreeDigest;
    firstFree(): bigint;
    pathForLeaf(index: bigint, leaf: Uint8Array): __compactRuntime.MerkleTreePath<Uint8Array>;
    findPathForLeaf(leaf: Uint8Array): __compactRuntime.MerkleTreePath<Uint8Array> | undefined;
    history(): Iterator<__compactRuntime.MerkleTreeDigest>
  };
  nullifiers: {
    isEmpty(): boolean;
    size(): bigint;
    member(elem: Uint8Array): boolean;
    [Symbol.iterator](): Iterator<Uint8Array>
  };
  readonly valueCom: { x: bigint, y: bigint };
  readonly contract: { bytes: Uint8Array };
}

export declare class Contract<T, W extends Witnesses<T> = Witnesses<T>> {
  witnesses: W;
  circuits: Circuits<T>;
  impureCircuits: ImpureCircuits<T>;
  constructor(witnesses: W);
  initialState(privateState: T): [T, __compactRuntime.ContractState];
}

export declare function ledger(state: __compactRuntime.StateValue): Ledger;
export declare const pureCircuits: PureCircuits;
