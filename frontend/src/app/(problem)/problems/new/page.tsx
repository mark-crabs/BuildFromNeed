"use client"

import { useState } from "react"
import Link from "next/link"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { ArrowLeft } from "lucide-react"

const categories = ["Organization", "Productivity", "Health", "Home", "Work", "Pets", "Food", "Technology", "Other"]

export default function NewProblem() {
  const [title, setTitle] = useState("")
  const [description, setDescription] = useState("")
  const [selectedCategory, setSelectedCategory] = useState("")
  const [tags, setTags] = useState<string[]>([])

  const handleAddTag = (tag: string) => {
    if (!tags.includes(tag) && tags.length < 5) {
      setTags([...tags, tag])
    }
  }

  const handleRemoveTag = (tag: string) => {
    setTags(tags.filter((t) => t !== tag))
  }

  const handleSubmit = () => {
    if (title.trim() && description.trim() && selectedCategory) {
      console.log("Problem submitted:", { title, description, category: selectedCategory, tags })
      // Reset form
      setTitle("")
      setDescription("")
      setSelectedCategory("")
      setTags([])
      // In real app, would navigate back
    }
  }

  const isValid = title.trim() && description.trim() && selectedCategory

  return (
    <>

      <main className="min-h-screen bg-gradient-to-br from-background via-background to-primary/5">
        <div className="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          {/* Back Button */}
          <Link href="/">
            <Button variant="ghost" className="gap-2 text-primary mb-6 hover:bg-primary/10">
              <ArrowLeft className="w-4 h-4" />
              Back to Home
            </Button>
          </Link>

          <Card className="bg-card border border-border p-8 shadow-sm">
            <h1 className="text-3xl font-bold text-foreground mb-2">Post Your Problem</h1>
            <p className="text-foreground/60 mb-8">
              Describe something you want to exist or solve. The community will suggest solutions.
            </p>

            {/* Problem Title */}
            <div className="mb-6">
              <label className="block text-sm font-semibold text-foreground mb-2">Problem Title</label>
              <input
                type="text"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                placeholder="e.g., How to organize cables behind desk"
                className="w-full px-4 py-3 rounded-lg border border-border bg-background text-foreground placeholder-foreground/50 focus:outline-none focus:ring-2 focus:ring-primary"
                maxLength={100}
              />
              <p className="text-xs text-foreground/50 mt-1">{title.length}/100</p>
            </div>

            {/* Category */}
            <div className="mb-6">
              <label className="block text-sm font-semibold text-foreground mb-3">Category</label>
              <div className="grid grid-cols-2 sm:grid-cols-3 gap-2">
                {categories.map((cat) => (
                  <button
                    key={cat}
                    onClick={() => setSelectedCategory(cat)}
                    className={`px-4 py-2 rounded-lg font-medium transition-all text-sm ${
                      selectedCategory === cat
                        ? "bg-primary text-primary-foreground shadow-md"
                        : "bg-muted text-foreground hover:bg-muted/80"
                    }`}
                  >
                    {cat}
                  </button>
                ))}
              </div>
            </div>

            {/* Problem Description */}
            <div className="mb-6">
              <label className="block text-sm font-semibold text-foreground mb-2">Detailed Description</label>
              <textarea
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Explain your problem in detail. What have you already tried? What's important to solve?"
                className="w-full px-4 py-3 rounded-lg border border-border bg-background text-foreground placeholder-foreground/50 focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                rows={6}
                maxLength={1000}
              />
              <p className="text-xs text-foreground/50 mt-1">{description.length}/1000</p>
            </div>

            {/* Tags */}
            <div className="mb-8">
              <label className="block text-sm font-semibold text-foreground mb-3">Add Tags (Optional)</label>
              <div className="flex flex-wrap gap-2 mb-4">
                {categories.map((cat) => (
                  <button
                    key={cat}
                    onClick={() => handleAddTag(cat)}
                    disabled={tags.includes(cat) || tags.length >= 5}
                    className={`px-3 py-1.5 text-xs rounded-full font-medium transition-all ${
                      tags.includes(cat)
                        ? "bg-accent text-accent-foreground"
                        : "bg-muted text-foreground hover:bg-muted/80 disabled:opacity-50"
                    }`}
                  >
                    {tags.includes(cat) ? "✓ " : "+ "}
                    {cat}
                  </button>
                ))}
              </div>
              {tags.length > 0 && (
                <div className="flex flex-wrap gap-2">
                  {tags.map((tag) => (
                    <button
                      key={tag}
                      onClick={() => handleRemoveTag(tag)}
                      className="px-3 py-1.5 text-sm font-medium bg-primary/20 text-primary rounded-lg hover:bg-primary/30 transition-colors"
                    >
                      {tag} ×
                    </button>
                  ))}
                </div>
              )}
            </div>

            {/* Submit Section */}
            <div className="flex gap-3 pt-6 border-t border-border">
              <Link href="/" className="flex-1">
                <Button variant="outline" className="w-full border-border bg-transparent">
                  Cancel
                </Button>
              </Link>
              <Button
                onClick={handleSubmit}
                disabled={!isValid}
                className="flex-1 bg-primary hover:bg-primary/90 text-primary-foreground disabled:opacity-50"
              >
                Post Problem
              </Button>
            </div>
          </Card>

          {/* Tips Section */}
          <Card className="bg-secondary/10 border border-secondary/30 p-6 mt-8">
            <h3 className="font-semibold text-foreground mb-3">Tips for a Great Problem Post</h3>
            <ul className="space-y-2 text-sm text-foreground/70">
              <li>• Be specific and clear about what you need</li>
              <li>• Mention what you've already tried</li>
              <li>• Include any constraints or requirements</li>
              <li>• Ask follow-up questions if solutions are unclear</li>
            </ul>
          </Card>
        </div>
      </main>
    </>
  )
}
