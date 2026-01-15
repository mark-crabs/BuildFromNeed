"use client"

import Link from "next/link"
import { Card } from "@/components/ui/card"
import { Avatar, AvatarImage, AvatarFallback } from "@/components/ui/avatar"
import { ThumbsUp, MessageCircle } from "lucide-react"
import { Problem } from "@/app/(problem)/problems/types"
import { timeAgo } from "@/app/utils"

export interface ProblemCardProps {
  problem: Problem
}

export function ProblemCard({ problem }: ProblemCardProps) {
  return (
    <Link href={`/problems/${problem.id}`}>
      <Card className="h-full hover:shadow-lg hover:border-primary/30 transition-all duration-300 hover:-translate-y-1 cursor-pointer group overflow-hidden bg-card border border-border">
        <div className="p-6 flex flex-col h-full">
          {/* Header */}
          <div className="flex items-start justify-between gap-4 mb-3">
            <div className="flex-1">
              <h3 className="font-bold text-lg leading-tight text-foreground group-hover:text-primary transition-colors line-clamp-2">
                {problem.title}
              </h3>
            </div>
          </div>

          {/* Description */}
          <p className="text-foreground/70 text-sm mb-4 line-clamp-2 flex-1">{problem.description}</p>

          {/* Tags */}
          {/* <div className="flex flex-wrap gap-2 mb-4">
            {problem.category  => (
              <span
                key={tag}
                className="inline-block px-2.5 py-1 text-xs font-medium bg-accent/20 text-accent rounded-full"
              >
                {tag}
              </span>
            ))}
            {problem.tags.length > 2 && (
              <span className="inline-block px-2.5 py-1 text-xs font-medium text-muted-foreground">
                +{problem.tags.length - 2}
              </span>
            )}
          </div> */}

          {/* Footer */}
          <div className="flex items-center justify-between pt-4 border-t border-border">
            <div className="flex items-center gap-2">
              <Avatar className="w-8 h-8">
                <AvatarImage src={problem.picture || "/placeholder.svg"} alt={problem.name} />
                <AvatarFallback>{problem.name.charAt(0)}</AvatarFallback>
              </Avatar>
              <div className="flex-1">
                <p className="text-xs font-medium text-foreground">{problem.name}</p>
                <p className="text-xs text-foreground/50">{timeAgo(problem.created_at)}</p>
              </div>
            </div>
          </div>

          {/* Stats */}
          <div className="flex gap-4 mt-4 pt-4 border-t border-border text-sm">
            <div className="flex items-center gap-1.5 text-foreground/70 group-hover:text-primary transition-colors">
              <ThumbsUp className="w-4 h-4" />
              <span className="font-medium">{problem.upvotes}</span>
            </div>
            <div className="flex items-center gap-1.5 text-foreground/70 group-hover:text-accent transition-colors">
              <MessageCircle className="w-4 h-4" />
              <span className="font-medium">{problem.solution_count}</span>
            </div>
          </div>
        </div>
      </Card>
    </Link>
  )
}
