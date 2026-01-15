import Link from "next/link"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { Avatar, AvatarImage, AvatarFallback } from "@/components/ui/avatar"
import { ThumbsUp, ThumbsDown, ArrowLeft, Heart, Share2 } from "lucide-react"
import { ProblemAndSolutions } from "../types"
import { timeAgo } from "@/app/utils"

export default async function ProblemDetail({ params }: { params: Promise<{ id: string }> }) {
  const { id } = await params
  const response = await fetch(`${process.env.BACKEND_DOMAIN}/problems/${id}`)
  const results = await response.json()
  const {problem, solution}: ProblemAndSolutions = results.data

  return (
    <>

      <main className="min-h-screen bg-gradient-to-br from-background via-background to-primary/5 dark:from-slate-900 dark:via-slate-900 dark:to-primary/10">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          {/* Back Button */}
          <Link href="/">
            <Button variant="ghost" className="gap-2 text-primary mb-6 hover:bg-primary/10">
              <ArrowLeft className="w-4 h-4" />
              Back to Problems
            </Button>
          </Link>

          {/* Problem Section */}
          <Card className="bg-card border border-border mb-8 p-8 shadow-sm">
            <div className="flex items-start justify-between mb-6">
              <h1 className="text-4xl font-bold text-foreground flex-1 pr-4">{problem.title}</h1>
              <div className="flex gap-2">
                <Button
                  variant="outline"
                  size="icon"
                  // onClick={() => setIsLiked(!isLiked)}
                  className={`border-border ${false ? "bg-accent text-accent-foreground" : ""}`}
                >
                  <Heart className={`w-5 h-5 ${false ? "fill-current" : ""}`} />
                </Button>
                <Button variant="outline" size="icon" className="border-border bg-transparent">
                  <Share2 className="w-5 h-5" />
                </Button>
              </div>
            </div>

            <p className="text-lg text-foreground/80 mb-6 leading-relaxed">{problem.description}</p>

            {/* Tags */}
            {/* <div className="flex flex-wrap gap-2 mb-8">
              {mockProblem.tags.map((tag) => (
                <span key={tag} className="px-3 py-1.5 text-sm font-medium bg-accent/20 text-accent rounded-full">
                  {tag}
                </span>
              ))}
            </div> */}

            {/* Author Info */}
            <div className="flex items-center justify-between pt-6 border-t border-border">
              <div className="flex items-center gap-3">
                <Avatar className="w-12 h-12">
                  <AvatarImage src={problem.picture || "/placeholder.svg"} alt={problem.name} />
                  <AvatarFallback>{problem.name.charAt(0)}</AvatarFallback>
                </Avatar>
                <div>
                  <p className="font-semibold text-foreground">{problem.name}</p>
                  <p className="text-sm text-foreground/60">{timeAgo(problem.created_at)}</p>
                </div>
              </div>
              <div className="flex items-center gap-3">
                <Button
                  variant="ghost"
                  size="sm"
                  // onClick={() => handleProblemVote("up")}
                  className={`gap-2 ${"up" === "up" ? "text-primary bg-primary/10" : "text-foreground/70"}`}
                >
                  <ThumbsUp className="w-5 h-5" />
                  <span className="font-bold">{problem.upvotes}</span>
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  // onClick={() => handleProblemVote("down")}
                  className={`gap-2 ${"down" === "down" ? "text-red-500 bg-red-500/10" : "text-foreground/70"}`}
                >
                  <ThumbsDown className="w-5 h-5" />
                  <span className="font-bold">{problem.downvotes}</span>
                </Button>
              </div>
            </div>
          </Card>

          {/* Solutions Section */}
          <div className="mb-12">
            <h2 className="text-3xl font-bold text-foreground mb-6">
              Solutions <span className="text-primary">({solution.length})</span>
            </h2>

            {/* New Solution Form */}
            <Card className="bg-gradient-to-br from-primary/10 to-accent/10 border border-primary/30 mb-8 p-6 shadow-sm">
              <h3 className="font-semibold text-foreground mb-4">Share Your Solution</h3>
              <textarea
                // value={newSolution}
                // onChange={(e) => setNewSolution(e.target.value)}
                placeholder="What's your solution to this problem? Be specific and helpful..."
                className="w-full p-4 rounded-lg border border-border bg-card text-foreground placeholder-foreground/50 focus:outline-none focus:ring-2 focus:ring-primary resize-none mb-4"
                rows={4}
              />
              <div className="flex justify-end gap-3">
                <Button variant="outline"  className="border-border">
                  Cancel
                </Button>
                <Button
                  // onClick={handleSubmitSolution}
                  // disabled={!newSolution.trim()}
                  className="bg-primary hover:bg-primary/90 text-primary-foreground"
                >
                  Post Solution
                </Button>
              </div>
            </Card>

            {/* Solutions List */}
            <div className="space-y-5">
              {solution.map((solution) => {
                // const votes = getSolutionVoteState(solution.id)
                return (
                  <Card
                    key={solution.id}
                    className="bg-card border border-border p-6 hover:border-primary/30 transition-colors"
                  >
                    <div className="flex items-start justify-between mb-4">
                      <div className="flex items-center gap-3 flex-1">
                        <Avatar className="w-10 h-10">
                          <AvatarImage src={solution.picture || "/placeholder.svg"} alt={solution.name} />
                          <AvatarFallback>{solution.name.charAt(0)}</AvatarFallback>
                        </Avatar>
                        <div>
                          <p className="font-semibold text-foreground">{solution.name}</p>
                          <p className="text-sm text-foreground/60">{timeAgo(solution.created_at)}</p>
                        </div>
                      </div>
                      {/* {solution.helpful && (
                        <span className="px-3 py-1 text-xs font-medium bg-secondary/20 text-secondary rounded-full">
                          Helpful
                        </span>
                      )} */}
                    </div>

                    <p className="text-foreground/80 mb-4 leading-relaxed">{solution.content}</p>

                    <div className="flex items-center justify-between pt-4 border-t border-border">
                      <div className="flex items-center gap-2">
                        <Button
                          variant="ghost"
                          size="sm"
                          // onClick={() => handleSolutionVote(solution.id, "up")}
                          className={`gap-2 ${"up" === "up" ? "text-primary bg-primary/10" : "text-foreground/70"}`}
                        >
                          <ThumbsUp className="w-4 h-4" />
                          <span className="font-medium">{solution.upvotes}</span>
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          // onClick={() => handleSolutionVote(solution.id, "down")}
                          className={`gap-2 ${"down" === "down" ? "text-red-500 bg-red-500/10" : "text-foreground/70"}`}
                        >
                          <ThumbsDown className="w-4 h-4" />
                          <span className="font-medium">{solution.downvotes}</span>
                        </Button>
                      </div>
                      <Button
                        variant="ghost"
                        size="sm"
                        className="text-foreground/70 hover:text-accent hover:bg-accent/10"
                      >
                        Mark Helpful
                      </Button>
                    </div>
                  </Card>
                )
              })}
            </div>
          </div>
        </div>
      </main>
    </>
  )
}
