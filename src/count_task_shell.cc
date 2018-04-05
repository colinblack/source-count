#include <fstream>
#include <cstring>
#include <cstdio>
#include <cstdlib>
#include "count_task_shell.h"
#include "public.h"

#define MAX_CHAR 1024
using std::ifstream;



/**
* ͳ�Ʋ���ӡ��������
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
		while ((*p == ' ' || *p == '\t')) ++p;    //����ǰ��ո�Tab
		const char* q = buf + strlen(buf) - 1;
		while ((q > p) && (*q == ' ' || *p == '\t')) --q;

		if(!(q - p + 1))
		{
			++_empty;  //����
		}
		if(!_comment_more)  //��ǰ״̬���Ƕ���ע��
		{
			if((pos_1 = strchr(p, '#')))   //����ע��
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
			else if(pusStakIfCommentMore(p)) //����Ƕ���ע�ͷ���ѹջ
			{
				_comment_more = true;
			}
			else if(poptakIfCommentMore(p) && _comment_stack.empty()) //����Ƕ���ע�ͷ�β��ջ
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



bool CCountTaskShell::pusStakIfCommentMore(const char* p)  //�Ƿ�Ϊ����ע�ͷ���
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


bool CCountTaskShell::poptakIfCommentMore(const char* p)  //�Ƿ�Ϊ����ע�ͷ�β
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


