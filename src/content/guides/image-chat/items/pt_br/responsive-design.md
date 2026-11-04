### Posicionamento Baseado em Porcentagem

O Image Chat usa coordenadas baseadas em porcentagem em vez de coordenadas em pixels para posicionar marcadores de chat nas imagens. Quando um usuário clica em uma imagem, o widget converte as coordenadas em pixels do clique em porcentagens da largura e altura da imagem. Essa abordagem garante que os marcadores permaneçam no local correto, independentemente de como a imagem é exibida.

Por exemplo, se um usuário clica a 250 pixels da borda esquerda de uma imagem de 1000px de largura, o widget armazena isso como 25% a partir da esquerda. Quando outro usuário visualiza a mesma imagem com 500px de largura em um dispositivo móvel, o marcador aparece a 125 pixels da esquerda, o que ainda corresponde a 25% da largura.

### Benefícios para Layouts Responsivos

Esse sistema de porcentagem faz com que o Image Chat funcione perfeitamente em todos os tamanhos e orientações de dispositivos. Suas imagens podem ser exibidas em tamanhos diferentes dependendo da largura da tela, das regras CSS ou das restrições do contêiner, mas os marcadores sempre se alinham corretamente aos locais pretendidos.

Usuários em computadores desktop com monitores grandes veem os marcadores nas mesmas posições relativas que usuários em smartphones com telas pequenas. Os marcadores escalam proporcionalmente com a própria imagem.

### Escala da Imagem e Proporção

Desde que sua imagem mantenha sua proporção ao ser escalada (que é o comportamento padrão do navegador), os marcadores baseados em porcentagem permanecerão perfeitamente alinhados. O widget assume que as imagens escalam proporcionalmente e calcula as posições com base nessa premissa.

Se você aplicar CSS que distorça a proporção da imagem (como usar `object-fit: cover` com dimensões específicas), as posições dos marcadores podem não se alinhar corretamente. Para obter os melhores resultados, permita que as imagens escalem naturalmente ou use `object-fit: contain` para manter a proporção.

### Dimensionamento do Quadrado de Chat

O tamanho visual dos marcadores de chat também é baseado em porcentagem. A opção de configuração `chatSquarePercentage` tem o padrão de 5%, o que significa que cada quadrado tem 5% da largura da imagem. Isso garante um peso visual consistente em diferentes tamanhos de imagem.

Em uma imagem de 1000px de largura com a configuração padrão de 5%, os marcadores têm 50px de lado. Em uma imagem de 500px de largura, os mesmos marcadores têm 25px de lado. Eles permanecem proporcionais ao tamanho da imagem.

### Comportamento em Dispositivos Móveis

Em telas com menos de 768px de largura, o Image Chat muda para um layout otimizado para dispositivos móveis. As janelas de chat abrem em tela cheia em vez de flutuar ao lado do marcador. Isso proporciona melhor usabilidade em telas pequenas, onde janelas flutuantes obscureceriam parte demais da imagem.

Os marcadores em si permanecem visíveis e clicáveis em suas posições baseadas em porcentagem. Os usuários ainda podem ver onde todas as discussões estão localizadas e tocar nos marcadores para abrir a interface de chat em tela cheia.

### Carregamento Dinâmico de Imagens

O sistema baseado em porcentagem funciona corretamente mesmo quando as imagens são carregadas de forma assíncrona ou mudam de tamanho após o carregamento da página. O widget monitora o elemento da imagem e recalcula as posições dos marcadores sempre que as dimensões da imagem mudam.

Se você estiver usando carregamento tardio (lazy-loading) de imagens ou implementando imagens responsivas com tamanhos diferentes em diferentes breakpoints, os marcadores se ajustam automaticamente quando o tamanho da imagem muda.

### Consistência entre Dispositivos

Como as coordenadas são armazenadas como porcentagens no banco de dados, uma discussão criada em um computador desktop aparece exatamente no mesmo local relativo quando visualizada em um tablet ou telefone. Os usuários podem colaborar entre dispositivos sem nenhuma inconsistência de posicionamento.

Isso funciona bidirecionalmente. Uma discussão criada ao tocar em um ponto específico em um dispositivo móvel aparece na mesma posição relativa quando visualizada em um monitor desktop grande.

### Considerações sobre o Viewport

O widget calcula as porcentagens em relação ao próprio elemento da imagem, não ao viewport. Isso significa que rolar a página ou alterar o tamanho da janela do navegador não afeta as posições dos marcadores. Os marcadores permanecem ancorados aos seus locais na imagem, independentemente das mudanças no viewport.

### Preparando o Conteúdo para o Futuro

A abordagem baseada em porcentagem torna suas discussões em imagens resilientes a mudanças de layout, design ou ecossistema de dispositivos. À medida que novos tamanhos de tela e dispositivos surgem, as discussões existentes continuarão a ser exibidas corretamente sem exigir atualizações ou migrações.