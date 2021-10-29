<script setup>
import { ref, computed, onMounted } from 'vue'
import { useWallet, useAnchorWallet } from '@solana/wallet-adapter-vue'
import { Program, Provider, web3 } from '@project-serum/anchor'
import { Connection, PublicKey } from '@solana/web3.js'
import idl from './assets/idl.json'

const programID = new PublicKey(idl.metadata.address)
const { publicKey, connected } = useWallet()
const counter = ref(null)
const fetchCounter = async () => {
    const account = await fetchAccount()
    counter.value = account.count.toString()
}
const create = async () => {
    await createCounter()
    await fetchCounter()
}
const increment = async () => {
    await incrementCounter()
    await fetchCounter()
}
const useWorkspace = () => workspaceStore
const initWorkspace = (network = 'https://api.devnet.solana.com', preflightCommitment = 'processed') => {
    const baseAccount = web3.Keypair.generate()
    const wallet = useAnchorWallet()
    const connection = new Connection(network, preflightCommitment)
    const provider = computed(() => new Provider(connection, wallet.value, { preflightCommitment }))
    const program = computed(() => new Program(idl, programID, provider.value))

    workspaceStore = {
        baseAccount,
        connection,
        provider,
        program,
        wallet,
    }
}
async function createCounter() {
    const { baseAccount, wallet, program } = useWorkspace()
    console.log("baseAccount: ", baseAccount)
    console.log("wallet: ", baseAccount)
    console.log("program: ", baseAccount)
    try {
        await program.value.rpc.create({
            accounts: {
                baseAccount: baseAccount.publicKey,
                user: wallet.value.publicKey,
                systemProgram: web3.SystemProgram.programId,
            },
            signers: [baseAccount]
        })
    } catch (err) {
        console.log("##Transaction error: ", err)
    }
} 
async function fetchAccount()  { 
    const { baseAccount, program } = useWorkspace()
    return await program.value.account.baseAccount.fetch(baseAccount.publicKey)
}
async function incrementCounter()  { 
    const { baseAccount, program } = useWorkspace()
    await program.value.rpc.increment({
        accounts: {
            baseAccount: baseAccount.publicKey
        }
    })
}

let workspaceStore = {}
onMounted(()=> {initWorkspace()})
</script>

<template>
    <div class="flex h-screen">
        <div class="m-auto w-full max-w-sm">
            <div class="flex flex-col -space-y-px">
                <div class="text-center bg-gray-50 border border-gray-200 rounded-t-lg p-8">
                    <h1 class="text-6xl font-semibold text-gray-700">{{ counter ?? 'NOT SET' }}</h1>
                    <p class="text-xs uppercase tracking-widest font-semibold text-gray-500">Counter</p>
                </div>
                <div class="flex bg-gray-50 border border-gray-200 rounded-b-lg overflow-hidden divide-x divide-gray-200">
                    <button class="flex-1 p-4 hover:bg-gray-100" @click="create">Create counter</button>
                    <button class="flex-1 p-4 hover:bg-gray-100" @click="increment">Increment counter</button>
                </div>
            </div>
            <p 
                class="text-xs text-gray-500 mt-4 text-center"
                v-text="connected ? `You are connected as: ${publicKey}` : 'You are not connected!'"
            ></p>
        </div>
    </div>
</template>
