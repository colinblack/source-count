#ifndef _COUNT_TASK_SHELL_H
#define _COUNT_TASK_SHELL_H
#include "count_task.h"
#include <stack>
using std::stack;


class CCountTaskShell :public CCountTask
{
public:
	CCountTaskShell(const string& path):
		_path(path), _total(0), _empty(0), _effective(0), _comment(0),
        _comment_more(false){}

	int count();
	bool pusStakIfCommentMore(const char* p);
	bool poptakIfCommentMore(const char* p);
	
private:
	const string _path;
	unsigned int _total;
	unsigned int _empty;
	unsigned int _effective;
	unsigned int _comment;
	bool  _comment_more;
	stack<const char*>  _comment_stack;   
	
};



#endif 
