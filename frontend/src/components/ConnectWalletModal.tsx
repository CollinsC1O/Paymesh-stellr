"use client";

import { useEffect } from "react";
import { createPortal } from "react-dom";
import { connect } from "@/hooks/stellar-wallets-kit";

interface ConnectWalletModalProps {
  isOpen: boolean;
  onClose: () => void;
  onConnectSuccess?: () => void;
}

export function ConnectWalletModal({
  isOpen,
  onClose,
  onConnectSuccess,
}: ConnectWalletModalProps) {
  const handleConnect = async () => {
    await connect(async () => {
      onConnectSuccess?.();
      onClose();
    });
  };

  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = "hidden";
      const handleEscape = (e: KeyboardEvent) => {
        if (e.key === "Escape") onClose();
      };
      window.addEventListener("keydown", handleEscape);
      return () => {
        document.body.style.overflow = "unset";
        window.removeEventListener("keydown", handleEscape);
      };
    }
  }, [isOpen, onClose]);

  if (!isOpen || typeof document === "undefined") return null;

  const modal = (
    <div
      className="fixed inset-0 z-[100] flex items-center justify-center p-4"
      style={{ backgroundColor: "rgba(0,0,0,0.85)" }}
      onClick={onClose}
      role="dialog"
      aria-modal="true"
      aria-labelledby="connect-wallet-modal-title"
      aria-describedby="connect-wallet-modal-description"
    >
      <div
        className="relative rounded-xl sm:rounded-2xl shadow-2xl p-6 sm:p-8"
        style={{
          width: "min(100%, 28rem)",
          minWidth: "320px",
          backgroundColor: "#0D0D10",
          border: "2px solid #232542",
        }}
        onClick={(e) => e.stopPropagation()}
      >
        <h2
          id="connect-wallet-modal-title"
          style={{
            color: "#DFDFE0",
            fontFamily: '"Anton", sans-serif',
            fontSize: "1.25rem",
            lineHeight: 1.25,
            letterSpacing: "0.05em",
            marginBottom: "0.75rem",
          }}
        >
          CONNECT YOUR WALLET
        </h2>
        <p
          id="connect-wallet-modal-description"
          style={{
            color: "#8398AD",
            fontSize: "0.9375rem",
            lineHeight: 1.6,
            marginBottom: "1.5rem",
          }}
        >
          You need to connect your wallet to access this page. Connect your
          Stellar wallet to continue.
        </p>

        <div
          className="flex flex-col-reverse sm:flex-row gap-3 sm:gap-4"
          style={{ marginTop: 0 }}
        >
          <button
            type="button"
            onClick={onClose}
            style={{
              flex: 1,
              padding: "0.75rem 1.5rem",
              borderRadius: "9999px",
              fontSize: "0.875rem",
              fontWeight: 900,
              letterSpacing: "0.05em",
              color: "#9EB3C9",
              border: "1px solid #232542",
              backgroundColor: "transparent",
              cursor: "pointer",
            }}
          >
            CANCEL
          </button>
          <button
            type="button"
            onClick={handleConnect}
            style={{
              flex: 1,
              padding: "0.75rem 1.5rem",
              borderRadius: "9999px",
              fontSize: "0.875rem",
              fontWeight: 900,
              letterSpacing: "0.05em",
              color: "#ffffff",
              backgroundColor: "#5B63D6",
              border: "none",
              cursor: "pointer",
            }}
          >
            CONNECT WALLET
          </button>
        </div>
      </div>
    </div>
  );

  return createPortal(modal, document.body);
}
