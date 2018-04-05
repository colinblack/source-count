#include <fstream>
#include <cstring>
#include <cstdio>
#include <cstdlib>
#include "count_task_shell.h"
#include "public.h"

#define MAX_CHAR 1024
using std::ifstream;



/**
* 统计并打印各种行数
*/
int  CCountTaskShell::count()
{
	ifstream read;
	read.open(_path.c_str(), ifstream::in);
	const char* pos_1 = NULL; 
	
	string line;
	char buf[MAX_CHAR] = { 0 };

	while (read.getline(buf, sizeof(buf)))
	{
		++_total;
		const char* p = buf;
		while ((*p == ' ' || *p == '\t')) ++p;    //过滤前后空格，Tab
		const char* q = buf + strlen(buf) - 1;
		while ((q > p) && (*q == ' ' || *p == '\t')) --q;

		if(!(q - p + 1))
		{
			++_empty;  //空行
		}
		if(!_comment_more)  //当前状态不是多行注释
		{
			if((pos_1 = strchr(p, '#')))   //单行注释
			{
				if(pos_1 == p)
				{
					++_comment;
				}
				else
				{
					++_comment;
					++_effective;
				}
			}
			else if(pusStakIfCommentMore(p)) //如果是多行注释符首压栈
			{
				_comment_more = true;
			}
			else if(poptakIfCommentMore(p) && _comment_stack.empty()) //如果是多行注释符尾出栈
			{
				_comment_more = false;
			}
			else
			{
				++_effective;
			}
		}
		else
		{
			++_comment;
		}
		memset(buf, 0, MAX_CHAR);
	}

	printf("file:%-25s total:%-10u empty:%-10u effective:%-10u comment:%-10u \n", _path.c_str(), _total, _empty, _effective, _comment);
	read.close();
	
	return 0;
}



bool CCountTaskShell::pusStakIfCommentMore(const char* p)  //是否为多行注释符首
{
	const char* pos = NULL;
	if((pos = strstr(p, ":<<BLOCK'")) ||(pos = strstr(p, ":<<'BLOCK"))
		||(pos = strstr(p, ":<<BLOCK"))||(pos = strstr(p, ":<<'")))
	{
		_comment_stack.push(p);
		return true;
	}
	else
	{
		return false;
	}
}


bool CCountTaskShell::poptakIfCommentMore(const char* p)  //是否为多行注释符尾
{
	const char* pos = NULL;
	if((pos = strstr(p, "BLOCK")) ||(pos = strstr(p, "'BLOCK"))
		||(pos = strstr(p, "BLOCK'")) || (pos = strchr(p, '\'')))
	{
		if(_comment_stack.size())
		{
			if(!strcmp(_comment_stack.top(), pos))
			{
				_comment_stack.top();
				return true;
			}
		}
		
		return false;
	}
	else
	{
		return false;
	}
}


