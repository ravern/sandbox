// From https://wiki.xxiivv.com/site/modal

#include <stdio.h>

typedef struct {
  char *a, *b;
} Rule;

static int rules_len, direction, queue;
static Rule rules[0x100];
static char dict[0x8000], *dict_ = dict;
static char bank_a[0x4000], *prog_ = bank_a;
static char bank_b[0x4000], *outp_ = bank_b;
static char *regs[0x100];

#define spacer(c) (c < 0x21 || c == '(' || c == ')')

static char *parse_rulefrag(char *line);

static char *walk(char *s) {
  char c;
  int depth = 0;
  if (s[0] == '(') {
    while ((c = *s++)) {
      if (c == '(')
        depth++;
      if (c == ')')
        --depth;
      if (!depth)
        return s;
    }
  }
  while (!spacer(s[0]) && (c = *s++))
    ;
  return s;
}

static int compare(char *a, char *b) {
  int i = 0, al = walk(a) - a, bl = walk(b) - b;
  if (al == bl)
    while (a[i] == b[i])
      if (!a[i] || ++i >= al)
        return 1;
  return 0;
}

static void call(char *s) {
  char *ss = walk(s);
  if (*s == '(')
    s++, --ss;
  while (s < ss)
    putc(*(s++), stdout);
}

static char *match(char *p, Rule *r) {
  int i;
  char c, *a = r->a, *b = p;
  for (i = 0x21; i < 0x7f; i++)
    regs[i] = 0;
  while ((c = *a)) {
    if (c == '?') {
      int id = (int)*(++a);
      if (regs[id]) {
        if (!compare(regs[id], b))
          return NULL;
      } else if (id == ':')
        call(b);
      else
        regs[id] = b;
      a++, b = walk(b), c = *b;
    }
    if (!*a && spacer(*b))
      return b;
    if (c != *b)
      return NULL;
    a++, b++;
  }
  return spacer(*b) ? b : NULL;
}

static int bind(char r, char *incoming) {
  int depth = 0;
  char c, *s = regs[(int)r];
  if (r == ':' && incoming != NULL)
    s = incoming, queue++;
  if (!s)
    return !fprintf(stderr, "?%c Empty\n", r);
  if (s[0] == '(') {
    while ((c = *s++)) {
      if (c == '(')
        depth++;
      *outp_++ = c;
      if (c == ')')
        --depth;
      if (!depth)
        return 1;
    }
  }
  while (!spacer(s[0]) && (*outp_++ = *s++))
    ;
  return 1;
}

static void save(int rule) {
  if ((direction = !direction))
    prog_ = bank_b, outp_ = bank_a;
  else
    prog_ = bank_a, outp_ = bank_b;
  if (rule >= 0) {
    char *program = direction ? bank_b : bank_a;
    while (program[1] && program[0] < 0x21)
      program++;
    fprintf(stderr, "%02d %s\n", rule, program);
  }
}

static char *parse_rulefrag(char *line) {
  int depth = 0;
  char c, *s = line, *res = dict_;
  if (s[0] == '(') {
    while ((c = *s++)) {
      if (c == '(') {
        depth++;
        if (depth == 1)
          continue;
      }
      if (c == ')') {
        --depth;
        if (!depth) {
          *dict_++ = 0;
          return res;
        }
      }
      *dict_++ = c;
    }
  }
  while (!spacer(s[0]) && (*dict_++ = *s++))
    ;
  *dict_++ = 0;
  return res;
}

static int rewrite(char *incoming) {
  char c, *p = direction ? bank_b : bank_a;
  while ((c = *p)) {
    int i;
    if (p[0] == '<' && p[1] == '>') {
      Rule *r = &rules[rules_len++];
      p += 3;
      r->a = parse_rulefrag(p), p = walk(p) + 1;
      r->b = parse_rulefrag(p), p = walk(p);
      while ((*outp_++ = *p++))
        ;
      save(-1);
      return 1;
    }
    if (p == bank_a || p == bank_b || spacer(*(p - 1))) {
      for (i = 0; i < rules_len; i++) {
        Rule *r = &rules[i];
        char *res = match(p, r);
        if (res != NULL) {
          char cc, *b = r->b;
          while ((cc = *b++)) {
            if (cc == '?')
              bind(*b++, incoming);
            else
              *outp_++ = cc;
          }
          while ((*outp_++ = *res++))
            ;
          *outp_++ = 0;
          save(i);
          return 1;
        }
      }
    }
    *outp_++ = c;
    p++;
  }
  *outp_++ = 0;
  return 0;
}

static void print_rules(void) {
  int i;
  fprintf(stderr, "\n");
  for (i = 0; i < rules_len; i++)
    fprintf(stderr, "<> (%s) (%s)\n", rules[i].a, rules[i].b);
  fprintf(stderr, "\n");
}

int main(int argc, char **argv) {
  FILE *f;
  if (argc < 2)
    return !printf("usage: modal [-v] source.modal\n");
  if (argc < 3 && argv[1][0] == '-' && argv[1][1] == 'v')
    return !printf("Modal - Modal Interpreter, 4 Apr 2024.\n");
  if (!(f = fopen(argv[1], "r")))
    return !printf("Invalid Modal file: %s.\n", argv[1]);
  queue = 2;
  fread(bank_a, 1, 0x1000, f), fclose(f);
  while (rewrite(argv[queue]))
    ;
  print_rules();
  return 0;
}