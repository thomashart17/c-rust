#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum CEnum {
  KValOne,
  KValTwo,
  KValThree,
} CEnum;

int32_t add(int32_t x, int32_t y);

uint32_t vec_test(void);

void modify_ptr(int32_t *n);

int32_t enum_param_test(enum CEnum param);

int32_t option_test(int32_t num);

void test(void);
