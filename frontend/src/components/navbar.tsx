"use client"

import Link from "next/link"
import { Button } from "@/components/ui/button"
import { Search, LogOut } from "lucide-react"
import { useState } from "react"
import { useRouter } from "next/navigation"

interface HeaderProps {
  isLoggedIn?: boolean
  userName?: string
}

export function Header({ isLoggedIn = false, userName = "User" }: HeaderProps) {
  const [isSearchOpen, setIsSearchOpen] = useState(false)
  const [showUserMenu, setShowUserMenu] = useState(false)
  const router = useRouter()

  const handleLogout = () => {
    // TODO: Connect to backend logout API
    console.log("Logging out...")
    router.push("/")
  }

  return (
    <header className="bg-gradient-to-r from-primary to-accent border-b border-border sticky top-0 z-40 shadow-sm">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-16">
          <Link href={isLoggedIn ? "/dashboard" : "/"} className="flex items-center gap-2">
            <div className="w-10 h-10 bg-primary-foreground rounded-lg flex items-center justify-center font-bold text-primary text-lg">
              BN
            </div>
            <span className="font-bold text-primary-foreground hidden sm:inline">BuildFromNeed</span>
          </Link>

          <div className="flex-1 max-w-xs mx-4 hidden md:block">
            <div className="relative">
              <input
                type="text"
                placeholder="Search problems..."
                className="w-full px-4 py-2 rounded-lg bg-primary-foreground/10 text-primary-foreground placeholder-primary-foreground/50 focus:outline-none focus:ring-2 focus:ring-accent"
              />
              <Search className="absolute right-3 top-2.5 w-5 h-5 text-primary-foreground/50" />
            </div>
          </div>

          <div className="flex items-center gap-2">
            <Button
              variant="ghost"
              size="icon"
              className="md:hidden text-primary-foreground hover:bg-primary-foreground/20"
            >
              <Search className="w-5 h-5" />
            </Button>

            {isLoggedIn ? (
              <div className="relative">
                <Button
                  variant="ghost"
                  className="text-primary-foreground hover:bg-primary-foreground/20"
                  onClick={() => setShowUserMenu(!showUserMenu)}
                >
                  <span className="text-sm font-medium">{userName}</span>
                </Button>
                {showUserMenu && (
                  <div className="absolute right-0 mt-2 w-48 bg-white rounded-lg shadow-lg border border-border z-50">
                    <Link href="/profile" className="block px-4 py-2 text-sm text-foreground hover:bg-gray-100">
                      View Profile
                    </Link>
                    <Link href="/settings" className="block px-4 py-2 text-sm text-foreground hover:bg-gray-100">
                      Settings
                    </Link>
                    <button
                      onClick={handleLogout}
                      className="w-full text-left px-4 py-2 text-sm text-destructive hover:bg-gray-100 flex items-center gap-2"
                    >
                      <LogOut className="w-4 h-4" />
                      Logout
                    </button>
                  </div>
                )}
              </div>
            ) : (
              <>
                <Link href="/auth/signin">
                  <Button
                    variant="outline"
                    className="border-primary-foreground text-primary-foreground hover:bg-primary-foreground/10 bg-transparent"
                  >
                    Sign In
                  </Button>
                </Link>
                <Link href="/auth/signup">
                  <Button className="bg-accent text-accent-foreground hover:bg-accent/90">Sign Up</Button>
                </Link>
              </>
            )}
          </div>
        </div>
      </div>
    </header>
  )
}
