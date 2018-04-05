#include <fstream>
#include <cstring>
#include <cstdio>
#include <cstdlib>
#include "count_task_cpp.h"
#include "public.h"

#define MAX_CHAR 1024
using std::ifstream;

/**
* 如果字符串外查找注释符
*/
const char* CCountTaskCPP::searstrOutOfString(const char* begin, const char* end, const char* src)
 {
	 const char* l = NULL;
	 const char* r = NULL;
	 const char*	ret = NULL;
	 ret = strstr(begin, src);

	 while (begin < end && ret)
	 {
		 l = strstr(begin, "\"");  //跳过转义符
		 l = findFistChrInRange(begin, l, '"'); //找到左引号
		 if (l && l < end)
		 {
			 r = l + 1;
			 while ((r = strchr(r, '"')) && *(r - 1) == '\\')  //找到右引号
			 {
				 r += 1;
			 }
			 if (r && r < end)
			 {
				if (ret < l)
				{				 		
					break;
			    }
				else if (ret > l && ret < r)
				{
				 	ret = strstr(r + 1, src);
				}
				else
				{
				 	begin = r + 1;
				}
		     }
			 else
			 {
				 break;
			 }	 
		}
		else
		{
			break;
		}
      }

	 return ret;
 }

/**
* 统计并打印各种行数
*/
int  CCountTaskCPP::count()
{
	ifstream read;
	read.open(_path.c_str(), ifstream::in);
	
	string line;
	char buf[MAX_CHAR] = { 0 };

	while (read.getline(buf, sizeof(buf)))
	{
		++_total;
		const char* p = buf;
		while ((*p == ' ' || *p == '\t')) ++p;    //过滤前后空格，Tab
		const char* q = buf + strlen(buf) - 1;
		while ((q > p) && (*q == ' ' || *p == '\t')) --q;

			int len = q - p + 1;
	
		if (_comment_flag_1)    //上一次状态是多行注释，每行有可能存在单行注释或多行注释，有效行状态
		{
			
			commentStatusPrase(p, len);  //多行注释状态处理

	
			if(_isComment  &&!_empty_line_flag) //当前状态为注释且不为空行
			{
				++_comment;
			}
	
			if (_effective_flag) //当前状态为有效行
			{
				++_effective;
				
			}	
		}
		else    
		{
			effectiveStatusPrase(p, len);  //非多行注释状态处理
			if (_isComment)
			{
				++_comment;
			}
	
			if (_effective_flag && !_empty_line_flag) //有效行且不是空行
			{
				++_effective;
			}
		}
		_isComment = false;
		_valid = false;
		memset(buf, 0, MAX_CHAR);
	}

	printf("file:%-25s total:%-10u empty:%-10u effective:%-10u comment:%-10u \n", _path.c_str(), _total, _empty, _effective, _comment);
	read.close();
	
	return 0;
}

/**
* 非多行注释状态处理
*/
int CCountTaskCPP::effectiveStatusPrase(const char* line, int len)
{
	if (!len)  //空行
	{
		++_empty;    
		_empty_line_flag = true;  //是空行
		_effective_flag = false;  //不是有效行
		return 0;
	}
	else
	{
		_empty_line_flag = false;
	}
	const char* pos_1 = searstrOutOfString(line, line + len, "//"); //范围内查找首次出现的字符，但是不找字符串内的
	const char* pos_2 = searstrOutOfString(line, line + len, "/*");

	if (pos_1 && pos_2 == NULL)
	{
		if (line < pos_1)
		{
			effectiveLinePrase(line, pos_1);  //是否存在有效行
		}
		else if (line == pos_1)
		{
			_effective_flag = false;	// //abc  非有效行		
		}
		_isComment = true;
		_comment_flag_1 = false;
	}
	else if (pos_1 == NULL && pos_2)
	{
		if (line < pos_2)
		{
			effectiveLinePrase(line, pos_2);
			
		}
		else if (line == pos_2)
		{
			_valid = true;
		}
		commentStatusPrase(pos_2 + 2, len - 2);  // 如果有/* 判断是多行注释还是单行注释，是不是有效行
		_isComment = true;
	
	}
	else if (pos_1 && pos_2)
	{
		if (pos_1 > pos_2)
		{
			if (line < pos_2)
			{
				effectiveLinePrase(line, pos_2);
				
			}
			else if (line == pos_2)
			{
				_valid = true;
			}
			commentStatusPrase(pos_2 + 2, len - 2);
		}
		else
		{
			if (line < pos_1)
			{
				effectiveLinePrase(line, pos_1);
			}
			else if (line == pos_1)
			{
				_effective_flag = false;
			}
			
			_comment_flag_1 = false;
		}
		_isComment = true;
	}
	else
	{
		_comment_flag_1 = false;
		effectiveLinePrase(line, line+len);
	}

	return 0;
}


/**
* 处理多行注释状态
*/
int CCountTaskCPP::commentStatusPrase(const char* line, int len)
{
	if (!len)
	{
		_empty_line_flag = true;
		_effective_flag = false;

		++_empty;                   //多行注释内的空行算作空行
		return 0;
	}
	else
	{
		_empty_line_flag = false;
	}
	_isComment = true;
	const char* tmp = line;
	const char* pos_1 = searstrOutOfString(line, line + len, "*/");

	if (pos_1)
	{
		line = pos_1 + 2;
		if (line == tmp + len)
		{
			_effective_flag = false;
			_comment_flag_1 = false;
			
		}
		else
		{
			effectiveStatusPrase(line, tmp + len - line);  //有可能是单行注释，或者多行注释，需继续找后面是否有/*或//
		}

	}
	else
	{
		if (_valid)
		{
			_effective_flag = false;
		}
		_comment_flag_1 = true;
	}

  
	return 0;
}
 
/**
* 处理有效字符串
*/
int CCountTaskCPP::effectiveLinePrase(const char* begin, const char* end)
{

	while ((*begin == ' ' || *begin == '\t')) ++begin;
	if (begin == end )
	{
		_effective_flag = false;
		return 0;
	}
	const char* pos = NULL;
	if ((pos = searstrOutOfString(begin, end, "\\")))  //如果是续行符，则不认为是有效行
	{
		_linkline_flag = true;
		_effective_flag = false;
	}
	else                             //是有效行
	{
		_effective_flag = true;
		_linkline_flag = false;
	}

	return 0;
}




