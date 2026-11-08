पोल को उसकी गिनती बनाए रखते हुए उसी स्थान पर संपादित करें: प्रश्न बदलें, किसी विकल्प का लेबल बदलें, उसे बंद या फिर से खोलें,
या बदलें कि मतदाताओं को कौन देख सकता है। विकल्पों को id द्वारा संबोधित किया जाता है - उन्हें जोड़ने, हटाने या पुनर्व्यवस्थित करने के लिए, पूरी सूची PUT करें।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| commentPollPatch | CommentPollPatch | हाँ |  |

## प्रतिक्रिया

रिटर्न: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'patchPoll उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_9876543210";

  const pollPatch: CommentPollPatch = {
    question: "Which new feature would you like to see?",
    options: [
      { id: "opt-1", label: "Dark Mode" },
      { id: "opt-2", label: "Multi-language Support" }
    ]
  };

  const response: SavePollResponse = await patchPoll(tenantId, commentId, pollPatch);
  console.log(response);
})();
[inline-code-end]