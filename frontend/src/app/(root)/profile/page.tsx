"use client"

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import Link from "next/link"

// Mock data - replace with real data from your backend
const mockUser = {
  id: "user-1",
  name: "Alex Chen",
  email: "alex@example.com",
  bio: "Full-stack developer passionate about solving problems",
  totalPoints: 1250,
  streakDays: 12,
  problemsPosted: 8,
  solutionsPosted: 15
}

export default function ProfilePage() {
  return (
    <main className="min-h-screen bg-gray-50">

      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Profile Header */}
        <Card className="bg-gradient-to-r from-purple-600 to-pink-600 border-0 text-white mb-8">
          <CardContent className="pt-8">
            <div className="flex items-start justify-between">
              <div className="flex-1">
                <div className="w-16 h-16 bg-white/20 rounded-full flex items-center justify-center text-4xl font-bold mb-4">
                  {mockUser.name[0]}
                </div>
                <h1 className="text-4xl font-bold">{mockUser.name}</h1>
                <p className="text-white/80 mt-2">{mockUser.email}</p>
                <p className="text-white/70 mt-2 max-w-lg">{mockUser.bio}</p>
              </div>
              <Link href="/settings">
                <Button variant="outline" className="border-white text-white hover:bg-white/20 bg-transparent">
                  Edit Profile
                </Button>
              </Link>
            </div>
          </CardContent>
        </Card>

        {/* Stats Section */}
        <div className="grid grid-cols-1 md:grid-cols-1 gap-8 mb-8">
          <Card className="bg-gradient-to-br from-blue-50 to-cyan-50 border-blue-200">
            <CardHeader>
              <CardTitle className="text-lg">Contribution Summary</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <div className="flex justify-between items-center pb-3 border-b">
                <span className="text-gray-700">Problems Posted</span>
                <span className="font-bold text-lg">{mockUser.problemsPosted}</span>
              </div>
              <div className="flex justify-between items-center pb-3 border-b">
                <span className="text-gray-700">Solutions Provided</span>
                <span className="font-bold text-lg">{mockUser.solutionsPosted}</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-700">Current Streak</span>
                <span className="font-bold text-lg text-orange-500">{mockUser.streakDays} 🔥</span>
              </div>
            </CardContent>
          </Card>
        </div>

      </div>
    </main>
  )
}
