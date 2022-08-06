#include "mruby.h"

void mruby_example_gem_init(mrb_state *mrb);

void mrb_mruby_example_gem_init(mrb_state *mrb)
{
  mruby_example_gem_init(mrb);
}

void mrb_mruby_example_gem_final(mrb_state *mrb) {}
