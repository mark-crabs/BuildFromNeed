"use client"

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { useState } from "react"


export default function SettingsPage() {
  const [notifications, setNotifications] = useState({
    emailNotifications: true,
    pushNotifications: false,
    weeklyDigest: true,
    solutionUpvotes: true,
  })

  const [theme, setTheme] = useState("light")

  return (
    <main className="min-h-screen bg-gray-50">

      <div className="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <h1 className="text-3xl font-bold text-foreground mb-8">Settings</h1>

        {/* Notification Settings */}
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>Notification Preferences</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center justify-between py-3 border-b">
              <div>
                <p className="font-semibold text-foreground">Email Notifications</p>
                <p className="text-sm text-foreground/60">Receive email updates about activity</p>
              </div>
              <input
                type="checkbox"
                checked={notifications.emailNotifications}
                onChange={(e) => setNotifications({ ...notifications, emailNotifications: e.target.checked })}
                className="w-5 h-5 rounded accent-primary"
              />
            </div>

            <div className="flex items-center justify-between py-3 border-b">
              <div>
                <p className="font-semibold text-foreground">Push Notifications</p>
                <p className="text-sm text-foreground/60">Receive browser notifications</p>
              </div>
              <input
                type="checkbox"
                checked={notifications.pushNotifications}
                onChange={(e) => setNotifications({ ...notifications, pushNotifications: e.target.checked })}
                className="w-5 h-5 rounded accent-primary"
              />
            </div>

            <div className="flex items-center justify-between py-3 border-b">
              <div>
                <p className="font-semibold text-foreground">Weekly Digest</p>
                <p className="text-sm text-foreground/60">Get a weekly summary of top problems</p>
              </div>
              <input
                type="checkbox"
                checked={notifications.weeklyDigest}
                onChange={(e) => setNotifications({ ...notifications, weeklyDigest: e.target.checked })}
                className="w-5 h-5 rounded accent-primary"
              />
            </div>

            <div className="flex items-center justify-between py-3">
              <div>
                <p className="font-semibold text-foreground">Solution Upvotes</p>
                <p className="text-sm text-foreground/60">Notify when your solutions get upvoted</p>
              </div>
              <input
                type="checkbox"
                checked={notifications.solutionUpvotes}
                onChange={(e) => setNotifications({ ...notifications, solutionUpvotes: e.target.checked })}
                className="w-5 h-5 rounded accent-primary"
              />
            </div>
          </CardContent>
        </Card>

        {/* Theme Settings */}
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>Appearance</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <label className="block text-sm font-semibold text-foreground mb-3">Theme</label>
              <select
                value={theme}
                onChange={(e) => setTheme(e.target.value)}
                className="w-full px-4 py-2 rounded-lg border border-border bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
              >
                <option value="light">Light</option>
                <option value="dark">Dark</option>
                <option value="auto">Auto (System)</option>
              </select>
            </div>
          </CardContent>
        </Card>

        {/* Account Settings */}
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>Account</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="py-3 border-b">
              <p className="font-semibold text-foreground mb-2">Email Address</p>
              <p className="text-foreground/60">alex@example.com</p>
            </div>

            <div className="py-3 border-b">
              <p className="font-semibold text-foreground mb-2">Password</p>
              <Button variant="outline" className="bg-transparent">
                Change Password
              </Button>
            </div>

            <div className="py-3">
              <p className="font-semibold text-foreground mb-2 text-destructive">Danger Zone</p>
              <Button variant="outline" className="border-destructive text-destructive hover:bg-red-50 bg-transparent">
                Delete Account
              </Button>
            </div>
          </CardContent>
        </Card>

        <div className="flex gap-3">
          <Button className="bg-primary hover:bg-primary/90">Save Changes</Button>
          <Button variant="outline" className="bg-transparent">
            Cancel
          </Button>
        </div>
      </div>
    </main>
  )
}
