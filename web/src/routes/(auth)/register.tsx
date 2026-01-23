import { Button } from "@base-ui/react/button";
import { Field } from "@base-ui/react/field";
import type { DeepKeys, ValidationError } from "@tanstack/react-form";
import { revalidateLogic, useForm } from "@tanstack/react-form";
import { createFileRoute } from "@tanstack/react-router";

interface FormValues {
  email: string;
  username: string;
  password: string;
  confirmPassword: string;
}

type FormFields = "email" | "username" | "password" | "confirmPassword";

export const Route = createFileRoute("/(auth)/register")({
  component: RouteComponent,
});

const formFields: Array<{
  name: FormFields;
  label: string;
  type: "email" | "password" | "text";
  onChangeValidator: (value: string) => string | undefined;
  placeholder?: string;
  description?: string;
}> = [
  {
    name: "email",
    label: "Email",
    type: "email",
    onChangeValidator: (value) =>
      !value
        ? "Email is required"
        : value.length < 3
          ? "Email must be at least 3 characters"
          : undefined,
    placeholder: "john.doe@gmail.com",
  },
  {
    name: "username",
    label: "Username",
    type: "text",
    onChangeValidator: (value) =>
      !value
        ? "Username is required"
        : value.length < 3
          ? "Username must be at least 3 characters"
          : undefined,
    placeholder: "",
  },
  {
    name: "password",
    label: "Password",
    type: "password",
    onChangeValidator: (value) =>
      !value
        ? "Password is required"
        : value.length < 8
          ? "Password must be at least 8 characters"
          : undefined,
    placeholder: "",
    description: "Choose a password with at least 8 characters",
  },
  {
    name: "confirmPassword",
    label: "Confirm password",
    type: "password",
    onChangeValidator: (value) =>
      !value
        ? "Confirm password is required"
        : value.length < 8
          ? "Confirm password must be at least 8 characters"
          : undefined,
    placeholder: "",
    description: "",
  },
];

function isEmpty(
  object: Partial<Record<DeepKeys<FormValues>, ValidationError>>,
) {
  for (const _ in object) {
    return false;
  }
  return true;
}

function RouteComponent() {
  const form = useForm({
    defaultValues: {
      email: "",
      username: "",
      password: "",
      confirmPassword: "",
    },
    validationLogic: revalidateLogic({
      mode: "submit",
      modeAfterSubmission: "change",
    }),
    validators: {
      onDynamic: ({ value: formValues }) => {
        const errors: Partial<Record<DeepKeys<FormValues>, ValidationError>> =
          {};

        if (formValues.password !== formValues.confirmPassword) {
          errors.password = "Password does not match your confirm password.";
          errors.confirmPassword =
            "Password does not match your confirm password.";
        }

        return isEmpty(errors) ? undefined : { form: errors, fields: errors };
      },
    },
    onSubmit: async ({ value }) => {
      return fetch("http://localhost:3000/auth/register", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(value),
      });
    },
  });

  return (
    <div>
      <h2 className="flex justify-center py-8 text-lg">Create your account</h2>

      <form
        className="flex max-x-3xs sm:mx-w-[20rem] flex-col gap-5 mx-4"
        onSubmit={(e) => {
          e.preventDefault();
          // e.stopPropagation();

          console.log("on form submit...");

          form.handleSubmit();
        }}
      >
        <div>
          {formFields.map((formField) => (
            <form.Field
              key={formField.name}
              name={formField.name}
              validators={{
                onChange: ({ value }) => formField.onChangeValidator(value),
                onChangeAsyncDebounceMs: 500,
              }}
              // biome-ignore lint/correctness/noChildrenProp: allow children props cause tanstack form
              children={(field) => (
                <Field.Root
                  className="flex flex-col mb-4"
                  name={field.name}
                  invalid={!field.state.meta.isValid}
                  dirty={field.state.meta.isDirty}
                  touched={field.state.meta.isTouched}
                >
                  <Field.Label className="font-light text-sm">
                    {formField.label}
                  </Field.Label>
                  <Field.Control
                    className="border rounded-xl p-3"
                    type={formField.type}
                    value={field.state.value}
                    onValueChange={field.handleChange}
                    onBlur={field.handleBlur}
                    placeholder={formField.placeholder}
                  />
                  {formField.description && (
                    <Field.Description className="font-light text-xs">
                      {formField.description}
                    </Field.Description>
                  )}
                  <Field.Error match={!field.state.meta.isValid}>
                    {field.state.meta.errors.join(",")}
                  </Field.Error>
                </Field.Root>
              )}
            />
          ))}
        </div>
        <Button
          className="flex items-center justify-center h-10 px-3.5 m-0 outline-0 border border-gray-200 rounded-md bg-gray-50 font-inherit text-base font-medium leading-6 text-gray-900 select-none hover:data-[disabled]:bg-gray-50 hover:bg-gray-100 active:data-[disabled]:bg-gray-50 active:bg-gray-200 active:shadow-[inset_0_1px_3px_rgba(0,0,0,0.1)] active:border-t-gray-300 active:data-[disabled]:shadow-none active:data-[disabled]:border-t-gray-200 focus-visible:outline focus-visible:outline-2 focus-visible:outline-blue-800 focus-visible:-outline-offset-1 data-[disabled]:text-gray-500"
          type="submit"
        >
          Create account
        </Button>
      </form>
    </div>
  );
}
