/**
 * \file cracked_shell.h
 * \author Adam Marshall
 */

#ifndef _MOD_CRACKED_SHELL

#define _MOD_CRACKED_SHELL

#include <unordered_map>

namespace cracked_shell {
  // TODO ShellArray, ShellHash, ShellFunction
  enum class ShellDataType {
    STRING, ARRAY, HASH, NUM, INT, FUNCTION
  };

  class ShellData {
    private:
      ShellDataType data_type;
      void * data;

    public:
      char*          to_string();
      int            to_integer();
      double         to_number();
      ShellArray*    to_array();
      ShellHash*     to_hash();
      ShellFunction* to_function();

      ShellData*     get_array(const int);
      ShellData*     get_hash(const char*);
      void           put_array(const int, ShellData*);
      void           put_hash(const char*, ShellData*);
      void           remove_array(const int);
      void           remove_hash(const char*);
      ShellDataType  get_type();
      int            size();

      bool           is_string();
      bool           is_integer();
      bool           is_number();
      bool           is_array();
      bool           is_hash();
      bool           is_function();
  };

  class ShellEnv {
    private:
      std::unordered_map<const char*, ShellData*> vars;
      ShellData** stack;
      int stackSize;

    public:
      void       get_var(const char*);
      void       set_var(const char*, ShellData*);
      ShellData* top();
      void       pop();
      void       push_number(double);
      void       push_integer(int);
      void       push_string(const char*);
      void       push_array(ShellArray*);
      void       push_hash(ShellHash*);
      void       push_function(ShellFunction*);
  };

  class CrackedShellApp {
    private:
      ShellEnv* env;

    public:
      CrackedShellApp(ShellEnv*);

      int run();
  };
}

#endif
