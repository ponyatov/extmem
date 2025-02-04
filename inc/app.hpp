#pragma once

/// @defgroup libc libc
/// @{
#include <cstdio>
#include <cstdlib>
#include <cassert>
/// @}

/// @defgroup app app
/// @brief application-specific components
/// @{



/// @}

/// @defgroup parser parser
/// @ingroup cli
/// @{
extern int yylex();
extern int yylineno;
extern char* yytext;
extern char* yyfile;
extern FILE* yyin;
extern int yyparse();
extern void yyerror(const char* msg);
#include "app.yacc.hpp"
/// @}
