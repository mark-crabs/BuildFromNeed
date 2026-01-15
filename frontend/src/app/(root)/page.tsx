
import { ProblemCard } from "@/components/problem-card"
import { Button } from "@/components/ui/button"
import { Plus, FilterX } from "lucide-react"
import Link from "next/link"
import { Problem } from "@/app/(problem)/problems/types"

const categories = ["All", "Organization", "Productivity", "Health", "Home", "Work", "Pets", "Food", "Sound"]


export  default async function Home() {
  const response = await fetch(`${process.env.BACKEND_DOMAIN}/problems`)
  const results = await response.json()
  const mockProblems: Problem[] = results['data']
  
  // const [selectedCategory, setSelectedCategory] = useState("All")
  // const [sortBy, setSortBy] = useState("recent")

  // const filteredProblems =
  //   selectedCategory === "All" ? mockProblems : mockProblems.filter((p) => p.tags.includes(selectedCategory))

  // const sortedProblems = [...filteredProblems].sort((a, b) => {
  //   if (sortBy === "popular") return b.upvotes - a.upvotes
  //   if (sortBy === "solutions") return b.solutionCount - a.solutionCount
  //   return 0
  // })

  return (
    <>
      <main className="min-h-screen bg-gradient-to-br from-background via-background to-primary/5">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
          {/* Hero Section */}
          
          <div className="mb-12">
            <h1 className="text-4xl sm:text-5xl font-bold bg-gradient-to-r from-primary via-accent to-secondary bg-clip-text text-transparent mb-4 text-balance">
              Share Problems, Find Solutions
            </h1>
            <p className="text-lg text-foreground/70 max-w-2xl text-balance mb-8">
              Have a problem you want solved? Post it here and let our community help you find creative solutions.
            </p>
            <Link href="/problems/new">
              <Button size="lg" className="gap-2 bg-primary hover:bg-primary/90 text-primary-foreground">
                <Plus className="w-5 h-5" />
                Post Your Problem
              </Button>
            </Link>
          </div>

          {/* Filter Section */}
          <div className="mb-10">
            <div className="bg-card border border-border rounded-xl p-6 shadow-sm">
              <h2 className="font-semibold text-foreground mb-4">Filter by Category</h2>
              <div className="flex flex-wrap gap-2 mb-6">
                {categories.map((cat) => (
                  <button
                    key={cat}
                    // onClick={() => setSelectedCategory(cat)}
                    // className={`px-4 py-2 rounded-lg font-medium transition-all ${
                    //   selectedCategory === cat
                    //     ? "bg-primary text-primary-foreground shadow-md"
                    //     : "bg-muted text-foreground hover:bg-muted/80"
                    // }`}
                  >
                    {cat}
                  </button>
                ))}
              </div>

              <div className="flex flex-wrap gap-4 items-center">
                <label className="font-medium text-foreground">Sort by:</label>
                <select
                  // value={sortBy}
                  // onChange={(e) => setSortBy(e.target.value)}
                  className="px-3 py-2 rounded-lg border border-border bg-card text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
                >
                  <option value="recent">Most Recent</option>
                  <option value="popular">Most Upvoted</option>
                  <option value="solutions">Most Solutions</option>
                </select>

                {/* {(selectedCategory !== "All" || sortBy !== "recent") && (
                  <button
                    onClick={() => {
                      setSelectedCategory("All")
                      setSortBy("recent")
                    }}
                    className="flex items-center gap-2 px-3 py-2 rounded-lg text-sm text-accent hover:bg-accent/10 transition-colors"
                  >
                    <FilterX className="w-4 h-4" />
                    Clear Filters
                  </button>
                )} */}
              </div>
            </div>
          </div>

          {/* Problems Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {mockProblems.length > 0 ? (
              mockProblems.map((problem) => <ProblemCard key={problem.id} problem={problem} />)
            ) : (
              <div className="col-span-full py-12 text-center">
                <p className="text-foreground/60 text-lg">No problems found in this category.</p>
              </div>
            )}
          </div>
        </div>
      </main>
    </>
  )
}
