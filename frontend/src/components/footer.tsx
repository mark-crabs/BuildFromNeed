"use client"

import Link from "next/link"
import { Coffee } from "lucide-react"

export function Footer() {
  return (
    <footer className="border-t border-border bg-background mt-16 py-8">
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div className="flex flex-col items-center justify-between gap-6 sm:flex-row">
          {/* Left side - About */}
          <div className="text-center sm:text-left">
            <h3 className="text-lg font-semibold text-foreground">BuildFromNeed</h3>
            <p className="mt-1 text-sm text-muted-foreground">
              You are paid in proportion to the magnitude of the problems that you solve - <em>Elon Musk</em>.
            </p>
          </div>

          {/* Right side - Links */}
          <div className="flex flex-col items-center gap-3 sm:flex-row sm:gap-6">
            <Link href="/about" className="text-sm text-muted-foreground transition-colors hover:text-foreground">
              About
            </Link>
            <Link href="/contact" className="text-sm text-muted-foreground transition-colors hover:text-foreground">
              Contact
            </Link>
            <Link
              href="https://buymeacoffee.com"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 rounded-lg bg-gradient-to-r from-yellow-400 to-orange-400 px-4 py-2 text-sm font-semibold text-black transition-all hover:shadow-lg hover:from-yellow-300 hover:to-orange-300"
            >
              <Coffee className="h-4 w-4" />
              Buy Me Coffee
            </Link>
          </div>
        </div>

        {/* Bottom - Copyright */}
        <div className="mt-8 border-t border-border pt-8 text-center">
          <p className="text-xs text-muted-foreground">
            © 2026 BuildFromNeed. All rights reserved. Made with ❤️ for the community.
          </p>
        </div>
      </div>
    </footer>
  )
}
