### Exemplo de formato

O CSV de Comentarios Programados se parece com isto:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Detalhes do formato

O arquivo CSV de Comentarios Programados suporta as seguintes colunas:

- ID
- Parent ID
- URL ID
- URL
- Name
- Avatar
- Comment
- Hours
- Minutes
- Seconds

As seguintes colunas sao **obrigatorias**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

Voce precisara da coluna Parent ID se quiser suportar respostas aninhadas automatizadas.

### Como o formato funciona

O formato de importacao funciona assim:

1. Voce define uma linha no CSV para cada comentario que deseja publicar.
2. O comentario e publicado apos o atraso especificado (horas + minutos + segundos).
   1. Para importacoes programadas manualmente, os atrasos sao relativos a quando voce pressiona "play" na interface, apos a importacao ser concluida - **nao quando a importacao comeca**.
   2. Para importacoes programadas automaticamente, o atraso e a partir do momento do carregamento da pagina.
3. Voce deve definir um ID. Voce pode simplesmente usar identificadores incrementais como 1, 2, 3, 4, 5.
4. Se voce quiser usar respostas aninhadas, basta definir o valor da coluna `Parent ID` para o `ID` de outro comentario.
5. O campo `Comment` suporta a mesma sintaxe que o FastComments suporta em seu widget de comentarios para estilizar texto e adicionar imagens.
6. O campo `Avatar`, se usado, deve ser uma imagem acessivel publicamente. Ela sera copiada e servida a partir do nosso CDN.
7. Voce pode excluir todos os comentarios apos a reproducao, ou se a reproducao for interrompida.
8. A exclusao acontece ao vivo, entao voce pode reutilizar a mesma importacao programada varias vezes.

### Um exemplo

[Um arquivo CSV de exemplo esta aqui](/csv/fastcomments-scheduled-comments-example.csv).
