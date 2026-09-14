A val은 자연스러운 웹훅 수신기입니다: 안정적인 URL을 가지고 있으며, 서명을 검증할 수 있고, SQLite와 블롭 스토리지를 내장하고 있습니다.

FastComments는 `${timestamp}.${body}`에 계정의 API 비밀키로 서명하고 두 개의 헤더를 전송합니다:

[inline-code-attrs-start title = '웹훅 헤더'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

이 메서드는 이벤트를 전달합니다: 생성되거나 업데이트된 댓글의 경우 **PUT**, 삭제된 경우 **DELETE**.

[inline-code-attrs-start title = '전달 검증'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // The exact bytes that arrived. Do NOT use c.req.json() and re-serialize.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // Reject stale deliveries so a captured request cannot be replayed later.
  if (Math.abs(Math.floor(Date.now() / 1000) - Number(timestamp)) > 300) {
    return new Response("Timestamp outside window", { status: 400 });
  }

  const expected = "sha256=" + createHmac("sha256", Deno.env.get("FASTCOMMENTS_API_SECRET"))
    .update(`${timestamp}.${rawBody}`)
    .digest("hex");

  const a = new TextEncoder().encode(signature);
  const b = new TextEncoder().encode(expected);
  if (a.length !== b.length || !timingSafeEqual(a, b)) {
    return new Response("Signature mismatch", { status: 401 });
  }

  // ...handle JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## 문제를 일으키는 두 가지

**원시 바이트를 검증하세요.** JSON을 파싱하고 다시 직렬화하면 키 순서와 공백이 바뀌어 해시가 달라지며, 그 결과 원인 없이 모든 전달이 실패합니다. 이것이 웹훅 수신기가 "그냥 작동하지 않는다"는 일반적인 이유입니다.

**상수 시간으로 비교하세요.** 서명에 대한 단순 `===` 연산은 일치한 바이트 수를 누출하게 되며, 이는 바이트를 하나씩 위조하는 데 충분합니다.

## 이벤트 처리

빠르게 응답하세요. FastComments는 2xx가 아닌 응답에 대해 재시도하며, 계속 실패하는 엔드포인트는 결국 자동으로 비활성화됩니다. 따라서 응답을 반환한 후에 실제 작업을 수행하고, 인라인으로 처리하지 마세요.

해당 작업을 댓글 ID에 대해 멱등하도록 만드세요. 재시도 시 새로운 타임스탬프로 다시 서명되며, 편집 및 삭제 시 동일한 댓글 ID가 다시 도착하므로 중복 제거를 위한 안정적인 기준이 없습니다.

---