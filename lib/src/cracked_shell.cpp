/**
 * \file cracked_shell.cpp
 * \author Adam Marshall
 */

#include "cracked_shell.h"

namespace cracked_shell {
  ShellData* ShellEnv::getenv(const char* name) {
    auto iter = vars.find(name);

    if (*iter == vars.end()) {
      return nullptr;
    }

    return iter->second;
  }

  void ShellEnv::setenv(const char* name, ShellData* datum) {
    vars.insert({ name, data });
  }

  CrackedShellApp::CrackedShellEnv(ShellEnv* _env): env(_env) {}

  int CrackedShellApp::run() {
    char*      line;
    char**     args;
    ParseTree* tree; // TODO
    int        status;

    env->get_var("PROMPT");
    
    const char * prompt_str = "$> ";
    if (env->top()->is_string()) {
      prompt_str = prompt->to_string();
    }
    
    do {
      std::cout << prompt_str;

      line = read_line();

      args = tokenize_line(line);

      tree = parse_tokens(args);

      status = execute_shell_program(tree);
    } while (status);
  }
}
