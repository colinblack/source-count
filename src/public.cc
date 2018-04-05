#include "public.h"
#include <cstring>

const char* strrpos(const char* dst, const char *src)
{
	char* ret = NULL;
	if (src == NULL)
	{
		return NULL;
	}

	const char* pos = strrchr(dst, *src);
	if (pos == NULL)
	{
		return NULL;
	}

	return strstr(pos, src);
}


const char* findStrInRange(const char* begin, const char* end, const char* src)
{
	const   char *pos = strrpos(begin, src);
	if (pos > end)
	{
		return NULL;
	}
	return pos;
}

const char* findChrInRange(const char* begin, const char* end, char src)
{
	const   char *pos = strrchr(begin, src);
	if (pos > end)
	{
		return NULL;
	}
	return pos;
}

const char* findFistChrInRange(const char* begin, const char* end, char src)
{
	const   char *pos = strchr(begin, src);
	if (pos > end)
	{
		return NULL;
	}
	return pos;
}

