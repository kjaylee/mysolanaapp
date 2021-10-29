<script setup>
import {
    getPhantomWallet,
    getSolflareWallet
} from '@solana/wallet-adapter-wallets'
import { WalletProvider } from '@solana/wallet-adapter-vue'
import { WalletMultiButton, WalletModalProvider } from '@solana/wallet-adapter-vue-ui'
import Counter from './Counter'
require("@solana/wallet-adapter-vue-ui/styles.css");

const wallets = [
    getPhantomWallet(),
    getSolflareWallet()
]
</script>

<template>
  <div class="full-screen">
    <wallet-provider :wallets="wallets" auto-connect>
        <div>
            <div class="fixed top-0 right-0 p-8">
                <wallet-modal-provider>
                    <wallet-multi-button></wallet-multi-button>
                </wallet-modal-provider>
            </div>
              <counter></counter>
        </div>
    </wallet-provider>
  </div>
</template>
