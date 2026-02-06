import { Button } from "@base-ui/react/button";
import { Field } from "@base-ui/react/field";
import { revalidateLogic, useForm } from "@tanstack/react-form";
import { createFileRoute } from "@tanstack/react-router";
import { useServerFn } from "@tanstack/react-start";
import { setCookieAndRedirect } from "~/fn/cookie";

export const Route = createFileRoute("/(auth)/login")({
  component: RouteComponent,
});

function RouteComponent() {
  const setCookieFn = useServerFn(setCookieAndRedirect);

  const form = useForm({
    defaultValues: {
      email: "",
      password: "",
    },
    validationLogic: revalidateLogic({
      mode: "submit",
      modeAfterSubmission: "change",
    }),
    validators: {
      onSubmitAsync: async ({ value: data, signal }) => {
        const res = await fetch("http://localhost:3000/auth/login", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(data),
          signal,
        });

        if (res.ok) {
          const data = await res.json();
          setCookieFn({ data: { access_token: data.access_token } });
        } else {
          // TODO: add failed message to Toast
          console.error("failed at logging in");
        }
      },
    },
  });
  return (
    <div>
      <h2>Login to your account</h2>
      <form
        className="flex max-x-3xs sm:mx-w-[20rem] flex-col gap-5 mx-4"
        onSubmit={(e) => {
          e.preventDefault();

          void form.handleSubmit();
        }}
      >
        <div>
          <form.Field
            key="email"
            name="email"
            // biome-ignore lint/correctness/noChildrenProp: allow children props
            children={(field) => (
              <Field.Root name="email" className="flex flex-col mb-4">
                <Field.Label className="font-light text-sm">Email</Field.Label>
                <Field.Control
                  className="border rounded-xl p-3"
                  type="text"
                  value={field.state.value}
                  onValueChange={field.handleChange}
                  onBlur={field.handleBlur}
                />
              </Field.Root>
            )}
          />
          <form.Field
            key="password"
            name="password"
            // biome-ignore lint/correctness/noChildrenProp: allow children props
            children={(field) => (
              <Field.Root name="password" className="flex flex-col mb-4">
                <Field.Label className="font-light text-sm">
                  Password
                </Field.Label>
                <Field.Control
                  className="border rounded-xl p-3"
                  type="password"
                  value={field.state.value}
                  onValueChange={field.handleChange}
                  onBlur={field.handleBlur}
                />
              </Field.Root>
            )}
          />
        </div>
        <Button type="submit">Login</Button>
      </form>
    </div>
  );
}
