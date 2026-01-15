"use client"

import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Lightbulb, Users, Target, Heart } from "lucide-react"
import Link from "next/link"

export default function AboutPage() {
  return (
    <main className="min-h-screen bg-background text-foreground">
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        {/* Hero Section */}
        <div className="text-center mb-16">
          <h1 className="text-4xl md:text-5xl font-bold mb-4 text-primary">About BuildFromNeed</h1>
          <p className="text-lg text-muted-foreground max-w-2xl mx-auto">
            A community-driven platform where problems meet solutions, and innovation happens through collaboration.
          </p>
        </div>

        {/* Mission Section */}
        <Card className="mb-12 border-2 border-primary/20">
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-primary">
              <Target className="w-6 h-6" />
              Our Mission
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4 text-base leading-relaxed">
            <p>
              BuildFromNeed exists because great solutions often come from unexpected places. We believe that the person
              facing a problem knows it better than anyone, and the solution might come from someone you've never met.
            </p>
            <p>
              Our platform democratizes problem-solving by giving everyone a voice. Whether you're struggling with a
              daily inconvenience or a complex challenge, we connect you with a community eager to help and contribute
              ideas.
            </p>
          </CardContent>
        </Card>

        {/* Core Values */}
        <div className="mb-12">
          <h2 className="text-3xl font-bold mb-8 text-center">Our Core Values</h2>
          <div className="grid md:grid-cols-2 gap-6">
            <Card className="border-2 border-secondary/20">
              <CardHeader>
                <CardTitle className="flex items-center gap-2 text-secondary">
                  <Lightbulb className="w-5 h-5" />
                  Creativity
                </CardTitle>
              </CardHeader>
              <CardContent>
                We celebrate out-of-the-box thinking and encourage creative solutions to everyday problems.
              </CardContent>
            </Card>

            <Card className="border-2 border-accent/20">
              <CardHeader>
                <CardTitle className="flex items-center gap-2 text-accent">
                  <Users className="w-5 h-5" />
                  Community
                </CardTitle>
              </CardHeader>
              <CardContent>
                Together, we're stronger. We foster a collaborative environment where everyone's ideas matter.
              </CardContent>
            </Card>

            <Card className="border-2 border-primary/20">
              <CardHeader>
                <CardTitle className="flex items-center gap-2 text-primary">
                  <Heart className="w-5 h-5" />
                  Impact
                </CardTitle>
              </CardHeader>
              <CardContent>
                Real solutions for real problems. We measure success by the positive changes we enable.
              </CardContent>
            </Card>

            <Card className="border-2 border-primary/20">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Users className="w-5 h-5" />
                  Inclusivity
                </CardTitle>
              </CardHeader>
              <CardContent>
                Everyone has something to contribute. We celebrate diverse perspectives and backgrounds.
              </CardContent>
            </Card>
          </div>
        </div>

        {/* How It Works */}
        <Card className="mb-12 border-2 border-primary/20">
          <CardHeader>
            <CardTitle>How It Works</CardTitle>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="flex gap-4">
              <div className="flex-shrink-0">
                <div className="flex items-center justify-center h-10 w-10 rounded-lg bg-primary text-primary-foreground font-bold">
                  1
                </div>
              </div>
              <div>
                <h3 className="font-semibold text-lg mb-2">Post Your Problem</h3>
                <p className="text-muted-foreground">
                  Describe a problem you're facing, no matter how big or small. Add context, tags, and details to help
                  others understand.
                </p>
              </div>
            </div>

            <div className="flex gap-4">
              <div className="flex-shrink-0">
                <div className="flex items-center justify-center h-10 w-10 rounded-lg bg-secondary text-secondary-foreground font-bold">
                  2
                </div>
              </div>
              <div>
                <h3 className="font-semibold text-lg mb-2">Get Solutions</h3>
                <p className="text-muted-foreground">
                  Community members share their ideas and solutions. Vote on the most helpful ones to highlight the best
                  answers.
                </p>
              </div>
            </div>

            <div className="flex gap-4">
              <div className="flex-shrink-0">
                <div className="flex items-center justify-center h-10 w-10 rounded-lg bg-accent text-accent-foreground font-bold">
                  3
                </div>
              </div>
              <div>
                <h3 className="font-semibold text-lg mb-2">Build Community</h3>
                <p className="text-muted-foreground">
                  Earn achievements, climb the leaderboard, and build your reputation as a trusted contributor and
                  problem-solver.
                </p>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* CTA */}
        <div className="text-center">
          <h2 className="text-2xl font-bold mb-4">Ready to Join the Community?</h2>
          <p className="text-muted-foreground mb-6">Start sharing your problems or solutions today.</p>
          <div className="flex gap-4 justify-center flex-wrap">
            <Link href="/problems/new">
              <Button size="lg" className="bg-primary hover:bg-primary/90">
                Post a Problem
              </Button>
            </Link>
            <Link href="/">
              <Button size="lg" variant="outline">
                Browse Problems
              </Button>
            </Link>
          </div>
        </div>
      </div>
    </main>
  )
}
