#include <fstream>
#include <cstring>
#include <cstdio>
#include <cstdlib>
#include "count_task_cpp.h"
#include "public.h"

#define MAX_CHAR 1024
using std::ifstream;

/**
* ����ַ��������ע�ͷ�
*/
const char* CCountTaskCPP::searstrOutOfString(const char* begin, const char* end, const char* src)
 {
	 const char* l = NULL;
	 const char* r = NULL;
	 const char*	ret = NULL;
	 ret = strstr(begin, src);

	 while (begin < end && ret)
	 {
		 l = strstr(begin, "\"");  //����ת���
		 l = findFistChrInRange(begin, l, '"'); //�ҵ�������
		 if (l && l < end)
		 {
			 r = l + 1;
			 while ((r = strchr(r, '"')) && *(r - 1) == '\\')  //�ҵ�������
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
* ͳ�Ʋ���ӡ��������
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
		while ((*p == ' ' || *p == '\t')) ++p;    //����ǰ��ո�Tab
		const char* q = buf + strlen(buf) - 1;
		while ((q > p) && (*q == ' ' || *p == '\t')) --q;

			int len = q - p + 1;
	
		if (_comment_flag_1)    //��һ��״̬�Ƕ���ע�ͣ�ÿ���п��ܴ��ڵ���ע�ͻ����ע�ͣ���Ч��״̬
		{
			
			commentStatusPrase(p, len);  //����ע��״̬����

	
			if(_isComment  &&!_empty_line_flag) //��ǰ״̬Ϊע���Ҳ�Ϊ����
			{
				++_comment;
			}
	
			if (_effective_flag) //��ǰ״̬Ϊ��Ч��
			{
				++_effective;
				
			}	
		}
		else    
		{
			effectiveStatusPrase(p, len);  //�Ƕ���ע��״̬����
			if (_isComment)
			{
				++_comment;
			}
	
			if (_effective_flag && !_empty_line_flag) //��Ч���Ҳ��ǿ���
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
* �Ƕ���ע��״̬����
*/
int CCountTaskCPP::effectiveStatusPrase(const char* line, int len)
{
	if (!len)  //����
	{
		++_empty;    
		_empty_line_flag = true;  //�ǿ���
		_effective_flag = false;  //������Ч��
		return 0;
	}
	else
	{
		_empty_line_flag = false;
	}
	const char* pos_1 = searstrOutOfString(line, line + len, "//"); //��Χ�ڲ����״γ��ֵ��ַ������ǲ����ַ����ڵ�
	const char* pos_2 = searstrOutOfString(line, line + len, "/*");

	if (pos_1 && pos_2 == NULL)
	{
		if (line < pos_1)
		{
			effectiveLinePrase(line, pos_1);  //�Ƿ������Ч��
		}
		else if (line == pos_1)
		{
			_effective_flag = false;	// //abc  ����Ч��		
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
		commentStatusPrase(pos_2 + 2, len - 2);  // �����/* �ж��Ƕ���ע�ͻ��ǵ���ע�ͣ��ǲ�����Ч��
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
* �������ע��״̬
*/
int CCountTaskCPP::commentStatusPrase(const char* line, int len)
{
	if (!len)
	{
		_empty_line_flag = true;
		_effective_flag = false;

		++_empty;                   //����ע���ڵĿ�����������
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
			effectiveStatusPrase(line, tmp + len - line);  //�п����ǵ���ע�ͣ����߶���ע�ͣ�������Һ����Ƿ���/*��//
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
* ������Ч�ַ���
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
	if ((pos = searstrOutOfString(begin, end, "\\")))  //��������з�������Ϊ����Ч��
	{
		_linkline_flag = true;
		_effective_flag = false;
	}
	else                             //����Ч��
	{
		_effective_flag = true;
		_linkline_flag = false;
	}

	return 0;
}




