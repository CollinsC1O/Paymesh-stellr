export default function WalletGuard() {
  return (
    <div className="flex items-center justify-center min-h-[60vh] w-full">
      <div
        className="animate-spin rounded-full h-12 w-12 border-2 border-[#232542] border-t-[#5B63D6]"
        aria-label="Loading"
      />
    </div>
  );
}
