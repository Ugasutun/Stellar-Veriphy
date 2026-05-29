import type { Metadata } from "next";
import { ToastProvider } from "@/app/context/ToastContext";

export const metadata: Metadata = {
  title: "StellarVeriphy",
  description: "Decentralized content verification on the Stellar blockchain",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <ToastProvider>{children}</ToastProvider>
      </body>
    </html>
  );
}
