import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/(app)/settings/preferences')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/(app)/settings/preference"!</div>
}
