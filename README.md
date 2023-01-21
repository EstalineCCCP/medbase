# ⚓MedBase - anotações e referências Unix-like
Um sistema simples de montar anotações referenciadas.

## Instalação
```
$ git clone 'https://github.com/EstalineCCCP/medbase'
$ cd medbase
$ cargo build --release
$ sudo mv -t /usr/local/sbin/ ./target/release/medbase mb
$ cp vim/ftdetect/medb.vim ~/.vim/ftdetect/ && cp vim/syntax/medb.vim ~/.vim/syntax/
```

Edite o script `mb`.

## Uso
Em um arquivo de texto simples, faça anotações seguidas de uma âncora. Para identificar a referência, use a âncora `::ref` no início da linha que delimita o fundo do bloco de notas:
```
Esta é uma anotação interessante sobre Diabetes Mellitus na página 20.
Esta é uma outra anotação sobre Diabetes Mellitus na página 24.
::ref AUTOR, Nome. Título do artigo ou livro. 2050. Editora. Cidade.
::loc /home/user/artigo.pdf
```
Chame o `medbase` usando standart input como streaming de entrada e os argumentos como termo da busca. Os termos podem ser separados por vírgula para busca composta:
```
$ cat '/caminho/para/uma-base-qualquer.medb' | medbase 'anotacao, interessante'
```
Ou no uso de arquivo único indicado pelo script `mb` (edite o script para indicar o arquivo .medb desejado):

```
mb 'diabetes anotacao interess'
```
Ambos exemplos resultará:
```
Esta é uma anotação interessante sobre Diabetes Mellitus na página 20.
⚓ Referência: AUTOR, Nome. Título do artigo ou livro. 2050. Editora. Cidade.
⚓ Localização: /home/user/artigo.pdf
```

O sistema tolera palavras incompletas, buscas de múltiplos termos e referências sem notas.

## Racional (abstração)
As âncoras são lançadas no fundo de cada nota. O fundo é uma linha em branco.

```
↓ Uma nota qualquer.
↓ Outra.
↓ Mais uma.
⚓::ref Âncora jogada no fundo da nota.
⚓::lin https://www.referenciadanota.com "Outra âncora jogada no fundo."

↓ Início de outra nota. A linha em branco acima é o fundo da nota anterior.
↓ As setas para baixo indicam o sentido da busca do MedBase.
⚓::ref Âncora jogada no fundo da nota.
⚓::lin https://www.referenciadanota.com "Outra âncora jogada no fundo."

```
## Vim
A sintaxe vim fornecida decora `âncoras`, comentários iniciador por `#` e anotações de páginas.

## Licença
> Copyright (c) 2022 - Jefferson T. 
> This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>
