"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { getPublicKey } from "@/hooks/stellar-wallets-kit";
import { ConnectWalletModal } from "./ConnectWalletModal";
import WalletGuard from "./WalletGuard";

export default function DashboardWalletGuard({
  children,
}: {
  children: React.ReactNode;
}) {
  const router = useRouter();
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [showModal, setShowModal] = useState(false);

  useEffect(() => {
    async function checkWallet() {
      const key = await getPublicKey();
      setPublicKey(key);
      if (!key) {
        setShowModal(true);
      }
      setIsLoading(false);
    }
    checkWallet();
  }, []);

  const handleConnectSuccess = async () => {
    const key = await getPublicKey();
    if (key) {
      setPublicKey(key);
      setShowModal(false);
    }
  };

  const handleClose = () => {
    setShowModal(false);
    router.push("/");
  };

  if (isLoading) {
    return <WalletGuard />;
  }

  if (!publicKey) {
    return (
      <>
        <div className="flex items-center justify-center min-h-[60vh]">
          <div className="text-center">
            <p className="text-[#5A6578] text-sm font-medium">
              Connect your wallet to continue
            </p>
          </div>
        </div>
        <ConnectWalletModal
          isOpen={showModal}
          onClose={handleClose}
          onConnectSuccess={handleConnectSuccess}
        />
      </>
    );
  }

  return <>{children}</>;
}
