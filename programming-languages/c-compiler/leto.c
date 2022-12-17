#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

bool isWhitespace(char c) {
  return c == '\r'
    || c == '\n'
    || c == ' '
    || c == '\t';
}

bool isDigit(char c) {
  return c >= '0' && c <= '9';
}

bool isLetter(char c) {
  return c == '_'
    || (c >= 'a' && c <= 'z')
    || (c >= 'A' && c <= 'Z');
}

typedef struct {
  int capacity;
  int length;
  char *content;
} Source;

Source *Source_new() {
  Source *s = (Source *)malloc(sizeof(Source));

  s->capacity = 0; // Just a random number
  s->length = 0;
  s->content = (char *)malloc(sizeof(char) * s->capacity);

  return s;
}

void Source_push(Source *s, char next) {
  if (s->length == s->capacity) {
    int oldCapacity = s->capacity;
    char *oldContent = s->content;

    s->capacity *= 2;
    s->content = (char *)malloc(sizeof(char) * s->capacity);
    memcpy(s->content, oldContent, oldCapacity);
  }

  s->content[s->length] = next;
  s->length++;
}

typedef struct {
  Source *source;
  int start;
  int end;
} Span;

char *Span_content(Span s) {
  int length = s.end - s.start;
  char *content = (char *)malloc(sizeof(char) * length);
  memcpy(content, &s.source->content[s.start], length);
  return content;
}

typedef enum {
  // Literals
  TOKEN_INT,
  TOKEN_IDENT,
  // Keywords
  TOKEN_RETURN,
  // Punctuation
  TOKEN_PLUS,
  TOKEN_DASH,
  TOKEN_OPAREN,
  TOKEN_CPAREN,
  TOKEN_OBRACE,
  TOKEN_CBRACE,
  TOKEN_SEMI,
  // Miscellaneous
  TOKEN_EOF,
} Token;

typedef struct {
  Token token;
  Span span;
} Lexeme;

typedef struct {
  FILE *input;
  Source *source;
  int start;
  int end;
  char current;
  bool isDone;
} Lexer;

Lexer *Lexer_new() {
  Lexer *l = (Lexer *)malloc(sizeof(Lexer));

  l->input = stdin;
  l->source = Source_new();
  l->start = 0;
  l->end = 0;
  l->current = 0;
  l->isDone = false;

  return l;
}

void Lexer_free(Lexer *l) {
  free(l);
}

char Lexer_peek(Lexer *l) {
  if (l->current != 0) {
    return l->current;
  }

  l->current = fgetc(l->input);
  Source_push(l->source, l->current);

  return Lexer_peek(l);
}

char Lexer_advance(Lexer *l) {
  char current = Lexer_peek(l);
  l->end++;
  l->current = 0;
  return current;
}

Span Lexer_buildSpan(Lexer *l) {
  Span span;
  
  span.source = l->source;
  span.start = l->start;
  span.end = l->end;

  return span;
}

Lexeme Lexer_buildLexeme(Lexer *l, Token token) {
  Lexeme lexeme;

  lexeme.span = Lexer_buildSpan(l);
  lexeme.token = token;

  return lexeme;
}

void Lexer_whitespace(Lexer *l) {
  while (isWhitespace(Lexer_peek(l))) {
    Lexer_advance(l);
  }
}

Lexeme Lexer_number(Lexer *l) {
  while (isDigit(Lexer_peek(l))) {
    Lexer_advance(l);
  }
  return Lexer_buildLexeme(l, TOKEN_INT);
}

Lexeme Lexer_ident(Lexer *l) {
  while (isDigit(Lexer_peek(l)) || isLetter(Lexer_peek(l))) {
    Lexer_advance(l);
  }

  char *content = Span_content(Lexer_buildSpan(l));

  if (strcmp(content, "return") == 0) return Lexer_buildLexeme(l, TOKEN_RETURN);
  else return Lexer_buildLexeme(l, TOKEN_IDENT);
}

Lexeme Lexer_next(Lexer *l) {
  if (l->isDone) {
    return Lexer_buildLexeme(l, TOKEN_EOF);
  }

  Lexer_whitespace(l);

  l->start = l->end;

  char c = Lexer_advance(l);
  switch (c) {
    case '+': return Lexer_buildLexeme(l, TOKEN_PLUS);
    case '-': return Lexer_buildLexeme(l, TOKEN_DASH);
    case '(': return Lexer_buildLexeme(l, TOKEN_OPAREN);
    case ')': return Lexer_buildLexeme(l, TOKEN_CPAREN);
    case '{': return Lexer_buildLexeme(l, TOKEN_OBRACE);
    case '}': return Lexer_buildLexeme(l, TOKEN_CBRACE);
    case ';': return Lexer_buildLexeme(l, TOKEN_SEMI);
    case EOF: {
      l->isDone = true;
      return Lexer_buildLexeme(l, TOKEN_EOF);
    }
  }

  if (isDigit(c)) {
    return Lexer_number(l);
  } else if (isLetter(c)) {
    return Lexer_ident(l);
  }
  
  exit(2);
}

int prefixBindingPower(Token token) {
  return -1;
}

int[2] infixBindingPower(Token token) {
  return [-1, -1];
}

int postfixBindingPower(Token token) {
  return -1;
}

typedef struct {
  FILE *output;
  Lexer *lexer;
  Lexeme current;
  bool hasCurrent;
} Compiler;

Compiler *Compiler_new() {
  Compiler *c = (Compiler *)malloc(sizeof(Compiler));

  c->output = stdout;
  c->lexer = Lexer_new();
  c->hasCurrent = false;

  return c;
}

Lexeme Compiler_peek(Compiler *c) {
  if (c->hasCurrent) {
    return c->current;
  }

  c->hasCurrent = true;
  c->current = Lexer_next(c->lexer);

  return c->current;
}

Lexeme Compiler_advance(Compiler *c) {
  Lexeme current = Compiler_peek(c);
  c->hasCurrent = false;
  return current;
}

Lexeme Compiler_expect(Compiler *c, Token token) {
  Lexeme current = Compiler_advance(c);
  if (current.token != token) {
    printf("unexpected %d, expected %d", current.token, token);
    exit(3);
  }
  return current;
}

void Compiler_literal_expression(Compiler *c) {
  Lexeme lexeme = Compiler_peek(c);
  if (lexeme.token == TOKEN_INT) {
    Lexeme lexeme = Compiler_advance(c);
    fprintf(c->output, " (i64.const %s)", Span_content(lexeme.span));
  } else {
    exit(5);
  }
}

void Compiler_pratt_expression(Compiler *c, int minPower) {
  switch (Compiler_peek(c).token) {
    case TOKEN_INT:
      Compiler_literal_expression(c);
      break;
    default:
      exit(6);
  }

  Lexeme lexeme = Compiler_peek(c);

  int postfixPower = postfixBindingPower(lexeme.token);
  if (postfixPower != -1) {
    if (postfixPower < minPower) {
      break;
    }

    continue;
  }

  int[2] infixPower = infixBindingPower(lexeme.token);
  int leftPower = infixPower[0];
  int rightPower = infixPower[1];
  if (leftPower != -1 && rightPower != -1) {
    if (leftPower < minPower) {
      break;
    }

    switch (lexeme.token) {
      case TOKEN_PLUS:
        i64.add
      case TOKEN_DASH:
    }

    continue;
  }

  break;
}

void Compiler_expression(Compiler *c) {
  Lexeme lexeme = Compiler_peek(c);

  switch (lexeme.token) {
    case TOKEN_INT: 
  }
}

void Compiler_return_statement(Compiler *c) {
  Compiler_expect(c, TOKEN_RETURN);
  fprintf(c->output, " (return");

  Compiler_expression(c);

  Compiler_expect(c, TOKEN_SEMI);
  fprintf(c->output, ")");
}

void Compiler_statement(Compiler *c) {
  Lexeme lexeme = Compiler_peek(c);

  if (lexeme.token == TOKEN_RETURN) {
    Compiler_return_statement(c);
  } else {
    exit(4);
  }
}

void Compiler_block(Compiler *c) {
  Compiler_expect(c, TOKEN_OBRACE);
  while (Compiler_peek(c).token != TOKEN_CBRACE) {
    Compiler_statement(c);
  }
  Compiler_expect(c, TOKEN_CBRACE);
}

void Compiler_type(Compiler *c, Lexeme type) {
  char *content = Span_content(type.span);

  if (strcmp(content, "int") == 0) fprintf(c->output, " i64");
}

void Compiler_declaration(Compiler *c) {
  fprintf(c->output, " (func");

  Lexeme type = Compiler_expect(c, TOKEN_IDENT);
  Lexeme name = Compiler_expect(c, TOKEN_IDENT);
  fprintf(c->output, " $%s (export \"%s\") (result", Span_content(name.span), Span_content(name.span));
  Compiler_type(c, type);
  fprintf(c->output, ")");

  Compiler_expect(c, TOKEN_OPAREN);
  Compiler_expect(c, TOKEN_CPAREN);

  Compiler_block(c);

  fprintf(c->output, ")");
}

void Compiler_compile(Compiler *c) {
  fprintf(c->output, "(module");
  while (Compiler_peek(c).token != TOKEN_EOF) {
    Compiler_declaration(c);
  }
  Compiler_expect(c, TOKEN_EOF);
  fprintf(c->output, ")\n");
}

int main() {
  Compiler *compiler = Compiler_new();

  Compiler_compile(compiler);

  return 0;
}
