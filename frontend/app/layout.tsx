import type { Metadata } from "next";
import { WalletProvider } from "@/context/WalletContext";
import { ToastProvider } from "@/app/context/ToastContext";
import { ThemeProvider } from "@/app/context/ThemeContext";
import { GeistSans } from "geist/font/sans";
import { GeistMono } from "geist/font/mono";
import { ThemeProvider } from "@/components/ThemeProvider";
import { ToastProvider } from "@/components/ToastProvider";
import { WalletProvider } from "@/components/WalletProvider";

export const metadata: Metadata = {
  title: "StellarVeriphy",
  description: "Decentralized content verification on the Stellar blockchain",
};

// Prevents FOUC: apply saved theme before first paint
const themeInitScript = `(function(){try{var t=localStorage.getItem('stellarproof-theme')||'dark';document.documentElement.classList.toggle('dark',t==='dark');document.documentElement.setAttribute('data-theme',t)}catch(e){}})();`;
// Prevents flash of unstyled content by applying the stored/preferred theme before paint
const themeInitScript = `(function(){try{var t=localStorage.getItem('theme')||(window.matchMedia('(prefers-color-scheme: dark)').matches?'dark':'light');document.documentElement.classList.toggle('dark',t==='dark')}catch(e){}})();`;

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <WalletProvider>{children}</WalletProvider>
        <ToastProvider>{children}</ToastProvider>
    <html lang="en" suppressHydrationWarning>
      <head>
        <script dangerouslySetInnerHTML={{ __html: themeInitScript }} />
      </head>
      <body>
        <ThemeProvider>{children}</ThemeProvider>
      <body className={`${GeistSans.variable} ${GeistMono.variable} font-sans`}>
        <ThemeProvider>
          <WalletProvider>
            <ToastProvider>{children}</ToastProvider>
          </WalletProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
