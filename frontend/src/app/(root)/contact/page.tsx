"use client"

import type React from "react"

import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Mail, Phone, MessageSquare, Send } from "lucide-react"
import { useState } from "react"

export default function ContactPage() {
  const [formData, setFormData] = useState({
    name: "",
    email: "",
    subject: "",
    message: "",
  })
  const [submitted, setSubmitted] = useState(false)
  const [isLoading, setIsLoading] = useState(false)

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }))
  }

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    setIsLoading(true)

    // Simulate sending email
    setTimeout(() => {
      setSubmitted(true)
      setFormData({ name: "", email: "", subject: "", message: "" })
      setIsLoading(false)

      // Reset success message after 5 seconds
      setTimeout(() => setSubmitted(false), 5000)
    }, 1500)
  }

  return (
    <main className="min-h-screen bg-background text-foreground">

      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        {/* Header */}
        <div className="text-center mb-12">
          <h1 className="text-4xl md:text-5xl font-bold mb-4 text-primary">Get in Touch</h1>
          <p className="text-lg text-muted-foreground max-w-2xl mx-auto">
            Have questions, feedback, or suggestions? We'd love to hear from you. Reach out anytime.
          </p>
        </div>

        <div className="grid md:grid-cols-3 gap-6 mb-12">
          {/* Contact Info Cards */}
          <Card className="border-2 border-primary/20 text-center">
            <CardHeader>
              <Mail className="w-8 h-8 mx-auto text-primary mb-2" />
              <CardTitle>Email</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground">
                <a href="mailto:hello@buildfromneed.com" className="hover:text-primary transition">
                  crabs@buildfromneed.com
                </a>
              </p>
            </CardContent>
          </Card>

          <Card className="border-2 border-secondary/20 text-center">
            <CardHeader>
              <MessageSquare className="w-8 h-8 mx-auto text-secondary mb-2" />
              <CardTitle>Discord</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground">
                <a href="#" className="hover:text-secondary transition">
                  Join Our Community
                </a>
              </p>
            </CardContent>
          </Card>

          <Card className="border-2 border-accent/20 text-center">
            <CardHeader>
              <Phone className="w-8 h-8 mx-auto text-accent mb-2" />
              <CardTitle>Support</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground">Response within 24 hours</p>
            </CardContent>
          </Card>
        </div>

        {/* Contact Form */}
        <Card className="border-2 border-primary/20 max-w-2xl mx-auto">
          <CardHeader>
            <CardTitle>Send us a Message</CardTitle>
          </CardHeader>
          <CardContent>
            {submitted && (
              <div className="mb-6 p-4 rounded-lg bg-green-50 dark:bg-green-950 border border-green-200 dark:border-green-800">
                <p className="text-green-800 dark:text-green-200 font-medium">
                  Thank you for your message! We'll get back to you soon.
                </p>
              </div>
            )}

            <form onSubmit={handleSubmit} className="space-y-6">
              <div>
                <label htmlFor="name" className="block text-sm font-medium mb-2">
                  Your Name
                </label>
                <input
                  type="text"
                  id="name"
                  name="name"
                  value={formData.name}
                  onChange={handleChange}
                  required
                  placeholder="John Doe"
                  className="w-full px-4 py-2 rounded-lg border border-border bg-input text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary"
                />
              </div>

              <div>
                <label htmlFor="email" className="block text-sm font-medium mb-2">
                  Your Email
                </label>
                <input
                  type="email"
                  id="email"
                  name="email"
                  value={formData.email}
                  onChange={handleChange}
                  required
                  placeholder="john@example.com"
                  className="w-full px-4 py-2 rounded-lg border border-border bg-input text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary"
                />
              </div>

              <div>
                <label htmlFor="subject" className="block text-sm font-medium mb-2">
                  Subject
                </label>
                <input
                  type="text"
                  id="subject"
                  name="subject"
                  value={formData.subject}
                  onChange={handleChange}
                  required
                  placeholder="How can we help?"
                  className="w-full px-4 py-2 rounded-lg border border-border bg-input text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary"
                />
              </div>

              <div>
                <label htmlFor="message" className="block text-sm font-medium mb-2">
                  Message
                </label>
                <textarea
                  id="message"
                  name="message"
                  value={formData.message}
                  onChange={handleChange}
                  required
                  placeholder="Tell us what's on your mind..."
                  rows={5}
                  className="w-full px-4 py-2 rounded-lg border border-border bg-input text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                />
              </div>

              <Button
                type="submit"
                size="lg"
                disabled={isLoading}
                className="w-full bg-primary hover:bg-primary/90 flex items-center justify-center gap-2"
              >
                <Send className="w-4 h-4" />
                {isLoading ? "Sending..." : "Send Message"}
              </Button>
            </form>
          </CardContent>
        </Card>

        {/* FAQ Section */}
        <div className="mt-16">
          <h2 className="text-3xl font-bold mb-8 text-center">Frequently Asked Questions</h2>
          <div className="space-y-4 max-w-2xl mx-auto">
            <Card className="border-2 border-primary/10">
              <CardHeader>
                <CardTitle className="text-lg">How long does it take to get a response?</CardTitle>
              </CardHeader>
              <CardContent className="text-muted-foreground">
                We typically respond to all inquiries within 24 hours during business days.
              </CardContent>
            </Card>

            <Card className="border-2 border-primary/10">
              <CardHeader>
                <CardTitle className="text-lg">Can I report a problem or abusive content?</CardTitle>
              </CardHeader>
              <CardContent className="text-muted-foreground">
                Yes! Use this contact form to report any issues. Include details about the problem and we'll investigate
                immediately.
              </CardContent>
            </Card>

            <Card className="border-2 border-primary/10">
              <CardHeader>
                <CardTitle className="text-lg">How can I partner with BuildFromNeed?</CardTitle>
              </CardHeader>
              <CardContent className="text-muted-foreground">
                We're always interested in collaborations. Reach out via email with partnership proposals or business
                inquiries.
              </CardContent>
            </Card>

            <Card className="border-2 border-primary/10">
              <CardHeader>
                <CardTitle className="text-lg">Is there a feature you'd like to see?</CardTitle>
              </CardHeader>
              <CardContent className="text-muted-foreground">
                Share your feature requests through this form. We read and consider all feedback from our community.
              </CardContent>
            </Card>
          </div>
        </div>
      </div>
    </main>
  )
}
