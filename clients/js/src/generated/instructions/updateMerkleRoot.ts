/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  bytes,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type UpdateMerkleRootInstructionAccounts = {
  tokenManager: PublicKey | Pda;
  authority?: Signer;
};

// Data.
export type UpdateMerkleRootInstructionData = {
  discriminator: Array<number>;
  merkleRoot: Uint8Array;
};

export type UpdateMerkleRootInstructionDataArgs = { merkleRoot: Uint8Array };

export function getUpdateMerkleRootInstructionDataSerializer(): Serializer<
  UpdateMerkleRootInstructionDataArgs,
  UpdateMerkleRootInstructionData
> {
  return mapSerializer<
    UpdateMerkleRootInstructionDataArgs,
    any,
    UpdateMerkleRootInstructionData
  >(
    struct<UpdateMerkleRootInstructionData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['merkleRoot', bytes({ size: 32 })],
      ],
      { description: 'UpdateMerkleRootInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [195, 173, 38, 60, 242, 203, 158, 93],
    })
  ) as Serializer<
    UpdateMerkleRootInstructionDataArgs,
    UpdateMerkleRootInstructionData
  >;
}

// Args.
export type UpdateMerkleRootInstructionArgs =
  UpdateMerkleRootInstructionDataArgs;

// Instruction.
export function updateMerkleRoot(
  context: Pick<Context, 'identity' | 'programs'>,
  input: UpdateMerkleRootInstructionAccounts & UpdateMerkleRootInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'soldIssuance',
    '3ja6s1Pb55nhzhwYp4GY77n972iEQtWX55xoRwP2asCT'
  );

  // Accounts.
  const resolvedAccounts = {
    tokenManager: {
      index: 0,
      isWritable: true as boolean,
      value: input.tokenManager ?? null,
    },
    authority: {
      index: 1,
      isWritable: false as boolean,
      value: input.authority ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: UpdateMerkleRootInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.authority.value) {
    resolvedAccounts.authority.value = context.identity;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getUpdateMerkleRootInstructionDataSerializer().serialize(
    resolvedArgs as UpdateMerkleRootInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
