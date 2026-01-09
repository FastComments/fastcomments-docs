### Percentage-Based Positioning

Image Chat usa coordenadas baseadas em porcentagem em vez de coordenadas em pixels para posicionar marcadores de conversa nas imagens. Quando um usuário clica em uma imagem, o widget converte as coordenadas em pixels do clique em porcentagens da largura e altura da imagem. Essa abordagem garante que os marcadores permaneçam na posição correta independentemente de como a imagem é exibida.

Por exemplo, se um usuário clicar a 250 pixels da borda esquerda de uma imagem de 1000px de largura, o widget armazena isso como 25% a partir da esquerda. Quando outro usuário visualiza a mesma imagem com 500px de largura em um dispositivo móvel, o marcador aparece a 125 pixels da esquerda, o que ainda é 25% da largura.

### Benefits for Responsive Layouts

Esse sistema baseado em porcentagem faz com que o Image Chat funcione perfeitamente em todos os tamanhos e orientações de dispositivo. Suas imagens podem ser exibidas em tamanhos diferentes dependendo da largura da tela, regras de CSS ou restrições do contêiner, mas os marcadores sempre se alinham corretamente com os locais pretendidos.

Usuários em computadores desktop com monitores grandes veem os marcadores nas mesmas posições relativas que usuários em smartphones com telas pequenas. Os marcadores escalam proporcionalmente com a própria imagem.

### Image Scaling and Aspect Ratio

Contanto que sua imagem mantenha sua proporção ao escalar (o que é o comportamento padrão do navegador), os marcadores baseados em porcentagem permanecerão perfeitamente alinhados. O widget presume que as imagens escalam proporcionalmente e calcula posições com base nessa suposição.

Se você aplicar CSS que distorce a proporção da imagem (como usar `object-fit: cover` com dimensões específicas), as posições dos marcadores podem não se alinhar corretamente. Para melhores resultados, permita que as imagens escalem naturalmente ou use `object-fit: contain` para manter a proporção.

### Chat Square Sizing

O tamanho visual dos marcadores de conversa também é baseado em porcentagem. A opção de configuração `chatSquarePercentage` tem padrão de 5%, o que significa que cada quadrado tem 5% da largura da imagem. Isso garante peso visual consistente entre diferentes tamanhos de imagem.

Em uma imagem de 1000px de largura com a configuração padrão de 5%, os marcadores têm 50px quadrados. Em uma imagem de 500px de largura, os mesmos marcadores têm 25px quadrados. Eles permanecem proporcionais ao tamanho da imagem.

### Mobile Behavior

Em telas com menos de 768px de largura, o Image Chat muda para um layout otimizado para dispositivos móveis. As janelas de conversa abrem em tela cheia em vez de flutuarem ao lado do marcador. Isso fornece melhor usabilidade em telas pequenas onde janelas flutuantes poderiam obscurecer grande parte da imagem.

Os próprios marcadores permanecem visíveis e clicáveis em suas posições baseadas em porcentagem. Os usuários ainda podem ver onde todas as discussões estão localizadas e tocar nos marcadores para abrir a interface de conversa em tela cheia.

### Dynamic Image Loading

O sistema baseado em porcentagem funciona corretamente mesmo quando as imagens são carregadas de forma assíncrona ou mudam de tamanho após o carregamento da página. O widget monitora o elemento da imagem e recalcula as posições dos marcadores sempre que as dimensões da imagem mudam.

Se você estiver fazendo lazy-loading de imagens ou implementando imagens responsivas com tamanhos diferentes em diferentes breakpoints, os marcadores se ajustam automaticamente quando o tamanho da imagem muda.

### Cross-Device Consistency

Como as coordenadas são armazenadas como porcentagens no banco de dados, uma discussão criada em um computador desktop aparece exatamente na mesma localização relativa quando visualizada em um tablet ou telefone. Os usuários podem colaborar entre dispositivos sem inconsistências de posicionamento.

Isso funciona bidirecionalmente. Uma discussão criada ao tocar em um ponto específico em um dispositivo móvel aparece na mesma posição relativa quando visualizada em um monitor de desktop grande.

### Viewport Considerations

O widget calcula porcentagens em relação ao próprio elemento da imagem, não ao viewport. Isso significa que rolar a página ou alterar o tamanho da janela do navegador não afeta as posições dos marcadores. Os marcadores permanecem ancorados às suas posições na imagem independentemente das mudanças no viewport.

### Future-Proofing Content

A abordagem baseada em porcentagem torna suas discussões em imagens resistentes a mudanças no layout, design ou ecossistema de dispositivos. À medida que novos tamanhos de tela e dispositivos surgirem, as discussões existentes continuarão a ser exibidas corretamente sem exigir atualizações ou migrações.