`tenantId: "demo"` is a shared public sandbox. It works with no signup, which is why the examples use it, but everyone else trying FastComments writes into the same threads and anyone can moderate them. Switch before you publish anything you care about.

Your tenant ID is on the [API secret page](https://fastcomments.com/auth/my-account/api-secret).

A tenant ID is public and belongs in browser code. An API secret does not, and nothing on this page needs one.

## Read it from an environment variable

Val Town vals are public on the free tier, so their source is world-readable. Keep anything sensitive in environment variables, read with `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

This matters more than usual on Val Town for a second reason: **remixing a val copies environment variable keys, but not their values.** A secret kept in an environment variable does not follow your val into someone else's account. A secret written into a file does.

Falling back to `"demo"` keeps the val working for anyone who remixes it before setting their own tenant.

## EU accounts

An account, its data, and its keys live in one region. If yours was created on `eu.fastcomments.com`, every widget config also needs `region: "eu"`, and the scripts load from `cdn-eu.fastcomments.com`. Otherwise leave both alone.
