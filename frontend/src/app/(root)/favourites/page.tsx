"use client"

import useSWR from "swr"
import { useState } from "react"
import { ProblemCard } from "@/components/problem-card"
import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import Link from "next/link"
import { ArrowLeft, Heart } from "lucide-react"

const mockFavorites = {
  problems: [
    {
        id: 3,
        anonymous: false,
        title: 'Improve productivity while working from home',
        description: 'Too many distractions and lose focus easily. Need strategies or tools to stay productive.',
        flag: 'NotReviewed',
        featured_id: 1,
        category: 'Sport',
        status: 'Trending',
        public: true,
        archive: false,
        created_at: '2026-01-14T12:21:10.519839',
        updated_at: '2026-01-14T12:21:10.519839',
        email: 'mumbijuanita@gmail.com',
        picture: "",
        name: 'Juanita Mumbi',
        upvotes: 0,
        downvotes: 0,
        solution_count: 0
      },
      {
        id: 2,
        anonymous: true,
        title: 'Better way to water indoor plants',
        description: 'Traveling for work often and plants keep dying. Looking for a low-maintenance watering solution.',
        flag: 'NotReviewed',
        featured_id: 1,
        category: 'Medical',
        status: 'Normal',
        public: true,
        archive: false,
        created_at: '2026-01-14T11:18:37.376230',
        updated_at: '2026-01-14T11:18:37.376230',
        email: 'Anonymous',
        picture: 'https://images.unsplash.com/photo-1582266255765-fa5cf1a1d501?q=80&w=2340&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        name: 'Anonymous',
        upvotes: 1,
        downvotes: 0,
        solution_count: 0
      },
      {
        id: 1,
        anonymous: false,
        title: 'How to organize cables behind desk',
        description: 'My desk is a mess of cables. Need a practical solution to keep them organized and accessible.',
        flag: 'NotReviewed',
        featured_id: 1,
        category: 'Sport',
        status: 'Trending',
        public: true,
        archive: false,
        created_at: '2026-01-13T18:06:53.141526',
        updated_at: '2026-01-13T18:06:53.141526',
        email: 'eugenemarkke@gmail.com',
        picture: 'https://lh3.googleusercontent.com/a/ACg8ocIolUsDXJ6T5WLLWaZKGN7IDtXL-XTy70AFFQqlmARI36ljD_WR=s96-c',
        name: 'Eugene Mark',
        upvotes: 1,
        downvotes: 0,
        solution_count: 2
      }
  ],
  solutions: [
    {
        "id": 2,
        "problem_id": 1,
        "content": "improve bro",
        "archive": false,
        "created_at": "2026-01-14T12:08:30.584700",
        "updated_at": "2026-01-14T12:08:30.584700",
        "upvotes": 1,
        "downvotes": 0,
        "email": "mumbijuanita@gmail.com",
        "picture": "",
        "name": "Juanita Mumbi"
    },
    {
        "id": 1,
        "problem_id": 1,
        "content": "good stuff",
        "archive": false,
        "created_at": "2026-01-14T11:26:40.058911",
        "updated_at": "2026-01-14T11:26:40.058911",
        "upvotes": 0,
        "downvotes": 1,
        "email": "eugenemarkke@gmail.com",
        "picture": "https://lh3.googleusercontent.com/a/ACg8ocIolUsDXJ6T5WLLWaZKGN7IDtXL-XTy70AFFQqlmARI36ljD_WR=s96-c",
        "name": "Eugene Mark"
    }
  ],
}

type TabType = "problems" | "solutions"

export default function FavoritesPage() {
  const [activeTab, setActiveTab] = useState<TabType>("problems")

  return (
    <>

      <main className="min-h-screen bg-gradient-to-br from-background via-background to-primary/5 dark:from-slate-900 dark:via-slate-900 dark:to-primary/10">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          {/* Back Button */}
          <Link href="/">
            <Button variant="ghost" className="gap-2 text-primary mb-6 hover:bg-primary/10">
              <ArrowLeft className="w-4 h-4" />
              Back to Home
            </Button>
          </Link>

          {/* Header */}
          <div className="mb-8">
            <h1 className="text-4xl font-bold text-foreground flex items-center gap-3">
              <Heart className="w-8 h-8 text-accent fill-accent" />
              Your Favorites
            </h1>
            <p className="text-foreground/60 mt-2">Your bookmarked problems and solutions</p>
          </div>

          {/* Tab Buttons */}
          <div className="flex gap-4 mb-8">
            <Button
              onClick={() => setActiveTab("problems")}
              className={`${
                activeTab === "problems"
                  ? "bg-primary text-primary-foreground"
                  : "bg-card border border-border text-foreground hover:bg-card/80"
              }`}
            >
              Problems ({mockFavorites.problems.length})
            </Button>
            <Button
              onClick={() => setActiveTab("solutions")}
              className={`${
                activeTab === "solutions"
                  ? "bg-primary text-primary-foreground"
                  : "bg-card border border-border text-foreground hover:bg-card/80"
              }`}
            >
              Solutions ({mockFavorites.solutions.length})
            </Button>
          </div>

          {/* Problems Tab */}
          {activeTab === "problems" && (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {mockFavorites.problems.map((problem) => (
                <div key={problem.id} className="relative">
                  <ProblemCard problem={problem} />
                  <Button variant="ghost" size="icon" className="absolute top-2 right-2 text-accent hover:bg-accent/10">
                    <Heart className="w-5 h-5 fill-accent" />
                  </Button>
                </div>
              ))}
            </div>
          )}

          {/* Solutions Tab */}
          {activeTab === "solutions" && (
            <div className="space-y-4">
              {mockFavorites.solutions.map((solution) => (
                <Card key={solution.id} className="bg-card border border-border p-6">
                  <div className="flex items-start justify-between mb-4">
                    <div className="flex-1">
                      <p className="text-sm text-foreground/60 mb-2">
                        <Link href={`/problems/${solution.problem_id}`} className="text-primary hover:underline">
                          {solution.content}
                        </Link>
                      </p>
                      <div className="flex items-center gap-3">
                        <img
                          src={solution.picture || "/placeholder.svg"}
                          alt={solution.name}
                          className="w-8 h-8 rounded-full"
                        />
                        <div>
                          <p className="font-semibold text-foreground text-sm">{solution.name}</p>
                          <p className="text-xs text-foreground/50">{solution.created_at}</p>
                        </div>
                      </div>
                    </div>
                    <Button variant="ghost" size="icon" className="text-accent hover:bg-accent/10">
                      <Heart className="w-5 h-5 fill-accent" />
                    </Button>
                  </div>

                  <p className="text-foreground/80 mb-4 leading-relaxed">{solution.content}</p>

                  <div className="flex items-center justify-between pt-4 border-t border-border">
                    <div className="flex gap-4 text-sm">
                      <span className="text-foreground/70">👍 {solution.upvotes}</span>
                      <span className="text-foreground/70">👎 {solution.downvotes}</span>
                    </div>
                    <Link href={`/problems/${solution.problem_id}`}>
                      <Button variant="outline" size="sm" className="border-border bg-transparent">
                        View Problem
                      </Button>
                    </Link>
                  </div>
                </Card>
              ))}
            </div>
          )}
        </div>
      </main>
    </>
  )
}
