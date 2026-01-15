"use client"

import type React from "react"

import { useState } from "react"
import Link from "next/link"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { Mail, Lock, User, ArrowLeft } from "lucide-react"

export default function SignUp() {
  const [name, setName] = useState("")
  const [email, setEmail] = useState("")
  const [password, setPassword] = useState("")
  const [confirmPassword, setConfirmPassword] = useState("")
  const [agreeTerms, setAgreeTerms] = useState(false)

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (password === confirmPassword) {
      console.log("Sign up:", { name, email, password })
      // TODO: Connect to backend API
    }
  }

  const handleGoogleSignUp = () => {
    console.log("Google sign up initiated")
    // TODO: Connect to Google OAuth
  }

  const handleGithubSignUp = () => {
    console.log("GitHub sign up initiated")
    // TODO: Connect to GitHub OAuth
  }

  const isValid = name && email && password && password === confirmPassword && agreeTerms

  return (
    <>
      <main className="min-h-screen bg-gradient-to-br from-background via-background to-primary/5 flex items-center">
        <div className="w-full max-w-md mx-auto px-4 sm:px-6">
          <Link href="/">
            <Button variant="ghost" className="gap-2 text-primary mb-8 hover:bg-primary/10 -ml-2">
              <ArrowLeft className="w-4 h-4" />
              Back
            </Button>
          </Link>

          <Card className="bg-card border border-border p-8 shadow-sm">
            <h1 className="text-2xl font-bold text-foreground mb-2">Join BuildFromNeed</h1>
            <p className="text-foreground/60 mb-8">Create an account to share solutions and help others</p>

            { /* 
            <form onSubmit={handleSubmit} className="space-y-4 mb-6">
              
              <div>
                <label className="block text-sm font-semibold text-foreground mb-2">Full Name</label>
                <div className="relative">
                  <User className="absolute left-3 top-3.5 w-5 h-5 text-foreground/50" />
                  <input
                    type="text"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    placeholder="John Doe"
                    required
                    className="w-full pl-10 pr-4 py-3 rounded-lg border border-border bg-background text-foreground placeholder-foreground/50 focus:outline-none focus:ring-2 focus:ring-primary"
                  />
                </div>
              </div>

              
              <div>
                <label className="block text-sm font-semibold text-foreground mb-2">Email Address</label>
                <div className="relative">
                  <Mail className="absolute left-3 top-3.5 w-5 h-5 text-foreground/50" />
                  <input
                    type="email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    placeholder="you@example.com"
                    required
                    className="w-full pl-10 pr-4 py-3 rounded-lg border border-border bg-background text-foreground placeholder-foreground/50 focus:outline-none focus:ring-2 focus:ring-primary"
                  />
                </div>
              </div>

              
              <div>
                <label className="block text-sm font-semibold text-foreground mb-2">Password</label>
                <div className="relative">
                  <Lock className="absolute left-3 top-3.5 w-5 h-5 text-foreground/50" />
                  <input
                    type="password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    placeholder="••••••••"
                    required
                    className="w-full pl-10 pr-4 py-3 rounded-lg border border-border bg-background text-foreground placeholder-foreground/50 focus:outline-none focus:ring-2 focus:ring-primary"
                  />
                </div>
              </div>

              
              <div>
                <label className="block text-sm font-semibold text-foreground mb-2">Confirm Password</label>
                <div className="relative">
                  <Lock className="absolute left-3 top-3.5 w-5 h-5 text-foreground/50" />
                  <input
                    type="password"
                    value={confirmPassword}
                    onChange={(e) => setConfirmPassword(e.target.value)}
                    placeholder="••••••••"
                    required
                    className="w-full pl-10 pr-4 py-3 rounded-lg border border-border bg-background text-foreground placeholder-foreground/50 focus:outline-none focus:ring-2 focus:ring-primary"
                  />
                </div>
                {password && confirmPassword && password !== confirmPassword && (
                  <p className="text-xs text-destructive mt-1">Passwords don't match</p>
                )}
              </div>

              
              <label className="flex items-start gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={agreeTerms}
                  onChange={(e) => setAgreeTerms(e.target.checked)}
                  className="w-4 h-4 rounded border-border accent-primary mt-1"
                />
                <span className="text-xs text-foreground/60">
                  I agree to the{" "}
                  <Link href="#" className="text-primary hover:text-primary/80 underline">
                    Terms of Service
                  </Link>{" "}
                  and{" "}
                  <Link href="#" className="text-primary hover:text-primary/80 underline">
                    Privacy Policy
                  </Link>
                </span>
              </label>

              
              <Button
                type="submit"
                disabled={!isValid}
                className="w-full bg-primary hover:bg-primary/90 text-primary-foreground font-medium py-3 mt-6 disabled:opacity-50"
              >
                Create Account
              </Button>
            </form>

            
            <div className="relative mb-6">
              <div className="absolute inset-0 flex items-center">
                <div className="w-full border-t border-border"></div>
              </div>
              <div className="relative flex justify-center text-sm">
                <span className="px-2 bg-card text-foreground/60">Or continue with</span>
              </div>
            </div>

            */ }

            {/* Social Auth */}
            <div className="grid grid-cols-1 gap-3 mb-6">
              <Button
                type="button"
                onClick={handleGoogleSignUp}
                variant="outline"
                className="border-border hover:bg-primary/5 bg-transparent transition-colors"
              >
                Google
              </Button>
              {/* <Button
                type="button"
                onClick={handleGithubSignUp}
                variant="outline"
                className="border-border hover:bg-primary/5 bg-transparent transition-colors"
              >
                GitHub
              </Button> */}
            </div>

            {/* Sign In Link */}
            <p className="text-center text-sm text-foreground/60">
              Already have an account?{" "}
              <Link href="/auth/signin" className="text-primary hover:text-primary/80 font-semibold">
                Sign in
              </Link>
            </p>
          </Card>
        </div>
      </main>
    </>
  )
}
