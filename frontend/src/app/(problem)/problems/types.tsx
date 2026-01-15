export interface Solution {
    id: number,
    problem_id: number
    user_id: number,
    content: string,
    archive: boolean,
    created_at: string,
    updated_at: string
    upvotes: number,
    downvotes: number,
    email: string,
    picture: string,
    name: string
}

export interface  Problem {
    id: number
    anonymous: boolean,
    title: string
    description: string
    flag: string,
    featured_id: number | undefined,
    category: string,
    status: string,
    public: boolean,
    archive: boolean,
    name: string,
    picture: string,
    upvotes: number,
    downvotes: number,
    created_at: string,
    updated_at: string,
    solution_count: number,
    email: string
}

export interface ProblemAndSolutions {
    problem: Problem,
    solution: Solution[]
}