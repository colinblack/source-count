#ifndef _COUNT_TASK_CPP_H
#define _COUNT_TASK_CPP_H
#include "count_task.h"


class CCountTaskCPP :public CCountTask
{
public:
	CCountTaskCPP(const string& path) :
		_path(path), _total(0), _empty(0), _effective(0), _comment(0),
		_effective_flag(false), _comment_flag_1(false), _linkline_flag(false), 
		_isComment(false), _empty_line_flag(false), _valid(false){}

	int count();
	int commentFlagIsTrue(const char* line, int len);
	int effectiveStatusPrase(const char* line, int len);
	int commentStatusPrase(const char* line, int len);
	int effectiveLinePrase(const char* begin, const char* end);
	const char* searstrOutOfString(const char* begin, const char* end, const char* src);

private:
	const string _path;
	unsigned int _total;
	unsigned int _empty;
	unsigned int _effective;
	unsigned int _comment;
	bool  _effective_flag;
	bool  _comment_flag_1;
	bool  _isComment;
	bool  _linkline_flag;
	bool  _empty_line_flag;
	bool  _valid;
};

#endif
