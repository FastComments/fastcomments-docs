## Exemplos de Zaps

Alguns fluxos de trabalho que levam minutos para configurar.

**Receba notificações sobre novos comentários.** Novo Comentário, então Slack "Send Channel Message" ou Discord "Send Channel Message". Mapeie o nome do comentarista, o texto do comentário e a URL da página na mensagem. Adicione o filtro de domínio para notificar um canal diferente por site.

**Mantenha um registro de cada comentário.** Novo Comentário, então Google Sheets "Create Spreadsheet Row". Adicione Deleted Comment como um segundo Zap que acrescenta uma linha com o ID do comentário, de modo que a planilha também funcione como trilha de auditoria.

**Envie e‑mail ao autor quando um comentário for aprovado.** Updated Comment com um filtro Zapier onde Approved é true, então Gmail "Send Email". Como Updated Comment dispara a cada alteração, o filtro é o que faz este Zap reagir apenas a aprovações.

**Adicione comentaristas ao seu CRM ou lista de e‑mail.** Novo Comentário, então HubSpot "Create or Update Contact" ou Mailchimp "Add or Update Subscriber" usando o e‑mail do comentarista. Respeite sua política de privacidade e a legislação local antes de adicionar alguém a uma lista de marketing.

**Crie um comentário a partir de um formulário.** Typeform ou Google Forms "New Response", então FastComments Create Comment com o ID da URL da página que seu site usa para depoimentos. Deixe Approved desmarcado para revisar cada um antes que apareça.

**Publique anúncios em um feed.** RSS by Zapier "New Item in Feed", então Create Feed Post com o título, conteúdo e link do item.

**Provisionar membros como usuários SSO.** Memberstack, Memberful ou seu próprio webhook, então Find SSO User seguido de Create SSO User no modo "find or create".

**Escalar comentários denunciados.** Updated Comment, filtrado com contagem de sinalizações acima de zero, então Trello "Create Card" ou Linear "Create Issue" com o ID do comentário e um link para a página de moderação.

**Publicar páginas à medida que são lançadas.** WordPress ou Ghost "New Post", então Create Page com a URL da postagem, de modo que a página seja listada e restrita antes do primeiro comentário.

**Arquivar comentários excluídos.** Deleted Comment, então Airtable "Create Record" com o comentário completo para retenção de conformidade.