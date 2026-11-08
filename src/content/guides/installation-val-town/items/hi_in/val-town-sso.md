यदि आपका val पहले से जानता है कि विज़िटर कौन है, तो Secure SSO उस पहचान को विजेट को सौंप देता है ताकि उन्हें दूसरा लॉगिन कभी न दिखे। बनाने के लिए कोई endpoints नहीं हैं और runtime पर कुछ भी कॉल करने की ज़रूरत नहीं है: आप तीन मान server-side पर compute करते हैं और उन्हें widget config में पास कर देते हैं।

Val Town `std/oauth` के साथ zero-config लॉगिन देता है, ताकि विज़िटर उसी Val Town खाते से साइन इन कर सके जो उनके पास पहले से है। इसे अपने ऐप द्वारा उपयोग किए जाने वाले किसी भी विकल्प से बदल दें; FastComments वाला हिस्सा नहीं बदलता।

## Payload को server पर बनाएँ

API secret payload पर हस्ताक्षर करता है और इसे कभी भी browser कोड तक नहीं पहुँचना चाहिए। SDK को npm से इंस्टॉल करें, जो Val Town के Deno runtime पर as-is काम करता है:

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // id एक ही व्यक्ति के लिए स्थिर होना चाहिए, अन्यथा उन्हें हर लॉगिन पर एक नई comment पहचान मिल जाती है।
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // email आवश्यक है और अद्वितीय होना चाहिए।
    email: user.email ?? `${id}@users.noreply.val.town`,
    // username आवश्यक है और ईमेल नहीं हो सकता।
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

`getPayload()` `{ userDataJSONBase64, verificationHash, timestamp }` लौटाता है। यही तीन मान browser तक पहुँचते हैं। Secret इन पर हस्ताक्षर करता है और फिर हटा दिया जाता है, इसलिए पेज में कुछ भी ऐसा नहीं होता जिससे कोई पाठक किसी भिन्न उपयोगकर्ता का forge कर सके।

## इसे विजेट को पास करें

[inline-code-attrs-start title = 'SSO के साथ Widget config'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { getOAuthUserData, oauthMiddleware } from "https://esm.town/v/std/oauth/middleware.ts";

app.get("/", async (c) => {
  const session = await getOAuthUserData(c.req.raw);
  const user = session?.user;

  const config = {
    tenantId: TENANT_ID,
    urlId: "my-thread",
    ...(user
      ? { sso: { ...buildSSOPayload(user), logoutURL: "/logout" } }
      : { sso: { loginURL: "/auth/login" } }),
  };

  // ...इस config के साथ विजेट render करें
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware` आपके लिए `GET /auth/login`, `GET /auth/callback` और `POST /auth/logout` जोड़ देता है। ध्यान दें कि logout एक **POST** है, जबकि विजेट `logoutURL` पर GET के साथ नेविगेट करता है, इसलिए `logoutURL` को अपने द्वारा बनाए गए एक छोटे route की ओर इंगित करें जो वह POST सबमिट करता है।

जब विज़िटर लॉग आउट हो, तो केवल `loginURL` के साथ `sso` पास करें। तब विजेट एक अनाम comment बॉक्स के बजाय लॉगिन प्रॉम्प्ट दिखाता है।

## जो चीज़ें गलत हो जाती हैं

`timestamp` epoch **milliseconds** में होना चाहिए, भविष्य में नहीं होना चाहिए, और दो दिन से अधिक पुराना नहीं होना चाहिए। इसे उसी request में server पर generate करें जो hash compute करती है। इसे browser में generate करना क्लासिक विफलता है: मान उससे भिन्न होता है जिस पर hash बना था और हर comment अस्वीकार कर दिया जाता है।

कभी भी identity provider से `isAdmin` या `isModerator` सेट न करें। Val Town खाते से साइन इन करने का इस बात से कोई लेना-देना नहीं है कि आपकी साइट कौन moderate करेगा।

पूरी field सूची, group-gated threads, और badges के लिए [SSO गाइड](/guide-customizations-and-configuration.html#sso-secure) देखें।