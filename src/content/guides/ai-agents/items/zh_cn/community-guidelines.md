编辑表单上的 **Community guidelines** 字段是一个可选的政策文本块，会作为用户角色上下文消息包含在该代理的每次运行中。它被作为不受信任的文本围栏（与平台对评论正文和其他用户提供内容所采用的相同围栏）处理，因此模型将其视为政策参考，而不是系统指令。它是记录“本网站允许和不允许的行为”的权威位置，以便代理一致地应用这些规则。

### 它与初始提示有何不同

- **Initial prompt** - 代理的角色与决策风格。 “你是版主。倾向于警告而不是封禁。”
- **Community guidelines** - 以政策语言表述的社区规则。 “不得进行人身攻击。注册不足 24 小时的账户不得发布推广链接。若讨论升温，或可移除与主题无关的评论。”

两者都会进入相同的上下文窗口，但它们进入的是不同的层级 —— 初始提示是系统角色的一部分，指南文档则是作为用户角色上下文消息中的被围栏文本。这样的分离在你想更新其中一项而不必重读另一项时使编辑更容易。

### 什么是好的指南文档

简短、具体、由人工撰写的文档。列表形式通常比散文更有效：

[inline-code-attrs-start title = '社区指南示例'; type='text' inline-code-attrs-end]
[inline-code-start]
Allowed:
- Substantive disagreement, even strongly worded.
- Links to original sources, even from new accounts.
- Off-topic asides if the parent thread permits them.

Not allowed:
- Personal attacks against specific named users.
- Doxxing or sharing of private information.
- Coordinated promotional activity (multiple comments pushing the same external link).
- Comments that exist only to derail discussion.

Borderline:
- Strong language without a target. Allowed if not directed at a person.
- Political topics outside the page subject. Off-topic; warn first, don't remove unless persistent.
[inline-code-end]

代理会在每次运行时应用这些规则。如果你更改了指南，该更改将在下一次触发时生效——过去的运行不会被追溯性地重新评估。

### 不要把什么放在这里

- **输出格式指令**（“以 HTML 回复”，“使用表情”）。这些应放在 [initial prompt](#personality-prompt) 中。
- **本地化文本。** 与提示一样，指南文档应为 **仅英文**，原因相同 —— 机器翻译可能会在不经意间更改代理行为。如果你的政策因地区而异，请在同一文档中全部用英文写出，并将文档结构化为 “for German-language pages: ...”。
- **外部政策的长篇引用。** 请改写。大量上下文会在每次运行时增加代价。
- **个人识别信息或秘密。** 该文本会在每次运行时发送到 LLM 提供方。

### 长度

该字段限制为 **4000 字符**（由表单和保存路由共同强制）。每次运行的 token 成本与长度成正比，因此即便在上限内，几百字通常已足够。如果你的社区政策多达数页，请将代理需要的部分总结成专门针对该字段的规范。

### 版本控制

指南文档没有内置的版本历史记录 —— 代理使用的是最后保存的值。如果你需要历史记录，请在每次重大编辑前将文档复制到你自己的跟踪系统中。[Refine Prompts](#refining-prompts) 流程可以记录对 *initial prompt* 的更改，但不会对指南文档进行版本化。

---